use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(string(Users::Username))
                    .col(string(Users::Role))
                    .col(string(Users::Password))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Feedbacks::Table)
                    .if_not_exists()
                    .col(pk_uuid(Feedbacks::Id))
                    .col(string(Feedbacks::Content))
                    .col(date_time(Feedbacks::CreatedAt))
                    .col(uuid_null(Feedbacks::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-feedbacks-user_id")
                            .from(Feedbacks::Table, Feedbacks::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feedbacks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Username,
    Role,
    Password,
}

#[derive(DeriveIden)]
enum Feedbacks {
    Table,
    Id,
    Content,
    CreatedAt,
    UserId,
}
