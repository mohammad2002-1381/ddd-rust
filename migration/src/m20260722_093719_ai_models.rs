use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260722_093719_ai_models"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AiModels::Table)
                    .if_not_exists()
                    .col(pk_auto(AiModels::Id))
                    .col(string(AiModels::Provider))
                    .col(string(AiModels::ModelName))
                    .col(integer(AiModels::MaxTokens))
                    .col(integer(AiModels::MaxInputTokens))
                    .col(boolean(AiModels::SupportsVision))
                    .col(boolean(AiModels::SupportsPromptCaching))
                    .col(double(AiModels::InputCostPerToken))
                    .col(double(AiModels::OutputCostPerToken))
                    .col(double_null(AiModels::CacheReadInputTokenCost))
                    .col(double_null(AiModels::CacheCreationInputTokenCost))
                    .col(timestamp_with_time_zone(AiModels::CreatedAt))
                    .col(timestamp_with_time_zone(AiModels::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AiModels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AiModels {
    #[sea_orm(iden = "ai_models")]
    Table,
    Id,
    Provider,
    ModelName,
    MaxTokens,
    MaxInputTokens,
    SupportsVision,
    SupportsPromptCaching,
    InputCostPerToken,
    OutputCostPerToken,
    CacheReadInputTokenCost,
    CacheCreationInputTokenCost,
    CreatedAt,
    UpdatedAt,
}
