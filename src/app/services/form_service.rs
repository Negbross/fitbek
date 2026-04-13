use crate::app::models::form::{
    CreateFormPayload, FormAnswerDto, FormDto, FormListItemDto, FormQuestionDto,
    FormResponseDto, SubmitResponsePayload
};
use entity::generated::{form_answers, form_questions, form_responses, forms};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, QueryOrder, Set, PaginatorTrait
};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::app::utils::error::AppError;

pub struct FormService;

impl FormService {
    fn generate_slug() -> String {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        let ms = since_the_epoch.as_millis();
        
        // Base62 encode a part of it to make a smallish slug
        let mut n = ms;
        let charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
        let mut slug = String::new();
        while n > 0 {
            slug.push(charset[(n % 62) as usize] as char);
            n /= 62;
        }
        
        slug.chars().rev().collect()
    }

    pub async fn create_form(
        db: &DatabaseConnection,
        user_id: uuid::Uuid,
        payload: CreateFormPayload,
    ) -> Result<String, AppError> {
        let slug = Self::generate_slug();

        let form = forms::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            title: Set(payload.title),
            description: Set(payload.description),
            url_slug: Set(slug.clone()),
            created_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        let form_res = form.insert(db).await?;

        for q in payload.questions {
            let question = form_questions::ActiveModel {
                id: Set(Uuid::new_v4()),
                form_id: Set(form_res.id.clone()),
                question_type: Set(q.question_type),
                label: Set(q.label),
                options: Set(q.options),
                is_required: Set(q.is_required),
                order_index: Set(q.order_index),
                ..Default::default()
            };
            question.insert(db).await?;
        }

        Ok(slug)
    }

    pub async fn get_forms(
        db: &DatabaseConnection,
        user_id: uuid::Uuid,
    ) -> Result<Vec<FormListItemDto>, AppError> {
        let forms_db = forms::Entity::find()
            .filter(forms::Column::UserId.eq(user_id))
            .order_by_desc(forms::Column::CreatedAt)
            .all(db)
            .await?;

        let mut dtos = Vec::new();
        for f in forms_db {
            let count = form_responses::Entity::find()
                .filter(form_responses::Column::FormId.eq(f.id.clone()))
                .count(db)
                .await?;

            dtos.push(FormListItemDto {
                id: String::from(f.id),
                title: f.title,
                url_slug: f.url_slug,
                response_count: count as i32,
            });
        }

        Ok(dtos)
    }

    pub async fn get_form_by_slug(
        db: &DatabaseConnection,
        slug: String,
    ) -> Result<Option<FormDto>, AppError> {
        let form_db = forms::Entity::find()
            .filter(forms::Column::UrlSlug.eq(slug))
            .one(db)
            .await?;

        if let Some(f) = form_db {
            let questions_db = form_questions::Entity::find()
                .filter(form_questions::Column::FormId.eq(f.id.clone()))
                .order_by_asc(form_questions::Column::OrderIndex)
                .all(db)
                .await?;

            let questions = questions_db
                .into_iter()
                .map(|q| FormQuestionDto {
                    id: String::from(q.id),
                    form_id: String::from(q.form_id),
                    question_type: q.question_type,
                    label: q.label,
                    options: q.options,
                    is_required: q.is_required,
                    order_index: q.order_index,
                })
                .collect();

            Ok(Some(FormDto {
                id: f.id,
                user_id: f.user_id,
                title: f.title,
                description: f.description,
                url_slug: f.url_slug,
                created_at: f.created_at.map(|d| d.to_string()),
                questions,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_form_responses(
        db: &DatabaseConnection,
        user_id: uuid::Uuid,
        form_id_str: String,
    ) -> Result<Option<Vec<FormResponseDto>>, AppError> {
        let form_id = uuid::Uuid::parse_str(&form_id_str).map_err(|_| AppError::BadRequest("Invalid form ID".to_string()))?;

        // Validate user owns this form
        let form_db = forms::Entity::find()
            .filter(forms::Column::Id.eq(form_id))
            .one(db).await?;
        if let Some(f) = form_db {
            if f.user_id != user_id {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        let responses_db = form_responses::Entity::find()
            .filter(form_responses::Column::FormId.eq(form_id))
            .order_by_desc(form_responses::Column::SubmittedAt)
            .all(db)
            .await?;

        let questions_db = entity::generated::form_questions::Entity::find()
            .filter(entity::generated::form_questions::Column::FormId.eq(form_id))
            .all(db)
            .await?;

        let mut question_map = std::collections::HashMap::new();
        for q in questions_db {
            question_map.insert(q.id.to_string(), q.label);
        }

        let mut response_dtos = Vec::new();
        for r in responses_db {
            let answers_db = form_answers::Entity::find()
                .filter(form_answers::Column::ResponseId.eq(r.id.clone()))
                .all(db)
                .await?;

            let answers = answers_db
                .into_iter()
                .map(|a| FormAnswerDto {
                    id: a.id.to_string(),
                    question_id: a.question_id.to_string(),
                    question_label: question_map.get(&a.question_id.to_string()).cloned(),
                    answer_value: a.answer_value,
                })
                .collect();

            response_dtos.push(FormResponseDto {
                id: r.id.to_string(),
                form_id: r.form_id.to_string(),
                submitted_at: r.submitted_at.map(|d| d.to_string()),
                answers,
            });
        }

        Ok(Some(response_dtos))
    }

    pub async fn submit_response(
        db: &DatabaseConnection,
        payload: SubmitResponsePayload,
    ) -> Result<(), AppError> {
        let parsed_form_id = uuid::Uuid::parse_str(&payload.form_id)
            .map_err(|_| AppError::BadRequest("Invalid form ID format".to_string()))?;

        let response = form_responses::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            form_id: Set(parsed_form_id),
            submitted_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };

        let response_res = response.insert(db).await?;

        for ans in payload.answers {
            let parsed_question_id = uuid::Uuid::parse_str(&ans.question_id)
                .map_err(|_| AppError::BadRequest("Invalid question ID format".to_string()))?;

            let answer = form_answers::ActiveModel {
                id: Set(uuid::Uuid::new_v4()),
                response_id: Set(response_res.id.clone()),
                question_id: Set(parsed_question_id), // String ID according to migration/entity
                answer_value: Set(ans.answer_value),
                ..Default::default()
            };
            answer.insert(db).await?;
        }

        Ok(())
    }

    pub async fn update_form(
        db: &DatabaseConnection,
        user_id: uuid::Uuid,
        payload: crate::app::models::form::UpdateFormPayload,
    ) -> Result<(), AppError> {
        let form_id = uuid::Uuid::parse_str(&payload.id)
            .map_err(|_| AppError::BadRequest("Invalid form ID".to_string()))?;

        let form_db = forms::Entity::find()
            .filter(forms::Column::Id.eq(form_id))
            .one(db).await?;

        if let Some(f) = form_db {
            if f.user_id != user_id {
                return Err(AppError::Forbidden);
            }
            let mut active_form: forms::ActiveModel = f.into();
            active_form.title = Set(payload.title);
            active_form.description = Set(payload.description);
            active_form.update(db).await?;
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn delete_form(
        db: &DatabaseConnection,
        user_id: uuid::Uuid,
        form_id_str: String,
    ) -> Result<(), AppError> {
        let form_id = uuid::Uuid::parse_str(&form_id_str)
            .map_err(|_| AppError::BadRequest("Invalid form ID".to_string()))?;

        let form_db = forms::Entity::find()
            .filter(forms::Column::Id.eq(form_id))
            .one(db).await?;

        if let Some(f) = form_db {
            if f.user_id != user_id {
                return Err(AppError::Forbidden);
            }
            f.delete(db).await?;
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}
