use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Forms::Table)
                    .if_not_exists()
                    .col(pk_uuid(Forms::Id))
                    .col(uuid(Forms::UserId))
                    .col(string(Forms::Title))
                    .col(string_null(Forms::Description))
                    .col(string(Forms::UrlSlug).unique_key())
                    .col(date_time_null(Forms::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-forms-user_id")
                            .from(Forms::Table, Forms::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FormQuestions::Table)
                    .if_not_exists()
                    .col(pk_uuid(FormQuestions::Id))
                    .col(uuid(FormQuestions::FormId))
                    .col(string(FormQuestions::QuestionType))
                    .col(string(FormQuestions::Label))
                    .col(text_null(FormQuestions::Options)) // JSON string for options
                    .col(boolean(FormQuestions::IsRequired).default(false))
                    .col(integer(FormQuestions::OrderIndex).default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-form_questions-form_id")
                            .from(FormQuestions::Table, FormQuestions::FormId)
                            .to(Forms::Table, Forms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FormResponses::Table)
                    .if_not_exists()
                    .col(pk_uuid(FormResponses::Id))
                    .col(uuid(FormResponses::FormId))
                    .col(date_time_null(FormResponses::SubmittedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-form_responses-form_id")
                            .from(FormResponses::Table, FormResponses::FormId)
                            .to(Forms::Table, Forms::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FormAnswers::Table)
                    .if_not_exists()
                    .col(pk_uuid(FormAnswers::Id))
                    .col(uuid(FormAnswers::ResponseId))
                    .col(uuid(FormAnswers::QuestionId))
                    .col(text_null(FormAnswers::AnswerValue))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-form_answers-response_id")
                            .from(FormAnswers::Table, FormAnswers::ResponseId)
                            .to(FormResponses::Table, FormResponses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-form_answers-question_id")
                            .from(FormAnswers::Table, FormAnswers::QuestionId)
                            .to(FormQuestions::Table, FormQuestions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FormAnswers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FormResponses::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(FormQuestions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Forms::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Forms {
    Table,
    Id,
    UserId,
    Title,
    Description,
    UrlSlug,
    CreatedAt,
}

#[derive(DeriveIden)]
enum FormQuestions {
    Table,
    Id,
    FormId,
    QuestionType,
    Label,
    Options,
    IsRequired,
    OrderIndex,
}

#[derive(DeriveIden)]
enum FormResponses {
    Table,
    Id,
    FormId,
    SubmittedAt,
}

#[derive(DeriveIden)]
enum FormAnswers {
    Table,
    Id,
    ResponseId,
    QuestionId,
    AnswerValue,
}
