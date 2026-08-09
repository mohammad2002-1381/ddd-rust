use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        // 1. Create Users Table
        manager.create_table(Table::create().table(Users::Table).if_not_exists()
            .col(pk_auto(Users::Id))
            .col(string(Users::FirstName))
            .col(string(Users::LastName))
            .col(string_uniq(Users::Email))
            .col(string(Users::PasswordHash))
            // Saved as a standard String/VARCHAR instead of a custom Postgres Enum
            .col(string(Users::Role).default("user"))
            .col(boolean(Users::IsActive))
            .col(timestamp_with_time_zone(Users::CreatedAt))
            .col(timestamp_with_time_zone(Users::UpdatedAt))
            .to_owned()
        ).await?;

        // 2. Create Tokens Table
        manager.create_table(Table::create().table(Tokens::Table).if_not_exists()
            .col(pk_auto(Tokens::Id))
            .col(string(Tokens::Token))
            .col(string(Tokens::RefreshToken))
            .col(integer(Tokens::UserId))
            .col(timestamp_with_time_zone(Tokens::Expires))
            .col(timestamp_with_time_zone(Tokens::CreatedAt))
            .col(timestamp_with_time_zone(Tokens::UpdatedAt))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_tokens_users")
                    .from(Tokens::Table, Tokens::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned()
        ).await?;

        // 3. Create Products Table
        manager.create_table(Table::create().table(Products::Table).if_not_exists()
            .col(pk_auto(Products::Id))
            .col(string(Products::Name))
            .col(big_integer(Products::Price))
            .col(boolean(Products::IsActive))
            .col(integer(Products::UserId))
            .col(timestamp_with_time_zone(Products::CreatedAt))
            .col(timestamp_with_time_zone(Products::UpdatedAt))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_products_users")
                    .from(Products::Table, Products::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned()
        ).await?;

        // 4. Create API Keys Table
        manager.create_table(Table::create().table(ApiKeys::Table).if_not_exists()
            .col(pk_auto(ApiKeys::Id))
            .col(string(ApiKeys::ApiKey))
            .col(big_integer(ApiKeys::UsedToken))
            .col(integer(ApiKeys::UserId))
            .col(boolean(ApiKeys::IsActive))
            .col(timestamp_with_time_zone(ApiKeys::CreatedAt))
            .col(timestamp_with_time_zone(ApiKeys::UpdatedAt))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_api_keys_users")
                    .from(ApiKeys::Table, ApiKeys::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Dropping tables in reverse order to avoid violating foreign key constraints
        manager.drop_table(Table::drop().table(ApiKeys::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Products::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Tokens::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    #[sea_orm(iden = "users")] Table,
    Id, FirstName, LastName, Email, PasswordHash, Role, IsActive, CreatedAt, UpdatedAt,
}

#[derive(DeriveIden)]
enum Tokens {
    #[sea_orm(iden = "tokens")] Table,
    Id, Token, RefreshToken, UserId, Expires, CreatedAt, UpdatedAt,
}

#[derive(DeriveIden)]
enum Products {
    #[sea_orm(iden = "products")] Table,
    Id, Name, Price, IsActive, UserId, CreatedAt, UpdatedAt,
}

#[derive(DeriveIden)]
enum ApiKeys {
    #[sea_orm(iden = "api_keys")] Table,
    Id, ApiKey, UsedToken, UserId, IsActive, CreatedAt, UpdatedAt,
}