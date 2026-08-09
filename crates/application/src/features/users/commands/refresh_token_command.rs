// use domain::{base_repository::IBaseRepository, models::users::{token, user_repository::IUserRepository}};
// use infrastructure::{base_repository::BaseRepository, queryable::IQueryable, repositories::user_repository::UserRepository};
// use sea_orm::{ColumnTrait, QueryFilter};
// use serde::Deserialize;

// use crate::{common::mediatr::{IRequest, IRequestHandler, RequestErrorType}, extensions::mapper::IMapper, features::users::dto::auth_dto::AuthDto, services::{current_user_service::{current_user_service::CurrentUserService, icurrent_user_service::ICurrentUserService}, get_services}};

// #[derive(Deserialize)]
// pub struct RefreshTokenCommand {
//     pub refresh_token: String
// }

// impl IRequest for RefreshTokenCommand {
//     type Response = AuthDto;
// }

// pub struct RefreshTokenCommandHandler<'a> {
//     pub token_repository: BaseRepository<token::Entity>,
//     pub user_repository: UserRepository,
//     pub current_user_service: &'a CurrentUserService
// }

// impl<'a> RefreshTokenCommandHandler<'a> {
//     pub fn new(current_user_service: &'a CurrentUserService) -> Self {
//         Self {
//             token_repository: BaseRepository::default(),
//             user_repository: UserRepository::new(),
//             current_user_service,
//         }
//     }
// }

// impl<'a> IRequestHandler<RefreshTokenCommand> for RefreshTokenCommandHandler<'a> {
//     async fn handle(&self, request: RefreshTokenCommand) -> Result<AuthDto, crate::common::mediatr::RequestErrorType> {
//         let user_id_option = self.current_user_service.user_id();
//         match user_id_option {
//             Some(user_id) => {
//                 let user_token_option = self.token_repository
//                 .get()
//                 .filter(token::Column::UserId.eq(user_id))
//                 .filter(token::Column::RefreshToken.eq(&request.refresh_token))
//                 .first_async().await?;
//                 match user_token_option {
//                     Some(user_token) => {
//                         self.token_repository.delete(user_token.id).await?;
//                         if !user_token.is_active() {
//                             return Err(RequestErrorType::Unauthorized("refresh token expired".to_string()))
//                         }
//                         let user_option = self.user_repository.find_by_id_async(user_id).await?;
//                         match user_option {
//                             Some(user) => {
//                                 let jwt_service = &get_services().jwt_service;
//                                 let token = jwt_service.generate_jwt_token(&user);
//                                 let refresh_token = jwt_service.generate_refresh_token();
//                                 let _ = self.token_repository.create(token::ActiveModel::new(&token, &refresh_token, user_id)).await?;
//                                 return Ok(AuthDto {
//                                     user: user.map(),
//                                     token,
//                                     refresh_token
//                                 })
//                             },
//                             None => return Err(RequestErrorType::NotFound("user not found in db".to_string())),
//                         }
//                     },
//                     None => return Err(RequestErrorType::NotFound("user token not found".to_string())),
//                 }
//             },
//             None => return Err(RequestErrorType::NotFound("user not found".to_string())),
//         }
//     }
// }