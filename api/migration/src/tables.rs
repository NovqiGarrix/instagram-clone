use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Users {
    Table,
    Id,
    Name,
    Username,
    Email,
    #[sea_orm(iden = "text")]
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Followers {
    Table,
    Id,
    UserId,
    FollowerId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Following {
    Table,
    Id,
    UserId,
    FollowingId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Stories {
    Table,
    Id,
    UserId,
    FileUrl,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Bookmarks {
    Table,
    Id,
    PostId,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Favorites {
    Table,
    Id,
    PostId,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum Posts {
    Table,
    Id,
    UserId,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum PostFiles {
    Table,
    Id,
    PostId,
    FileUrl,
}

#[derive(DeriveIden)]
pub enum PostComments {
    Table,
    Id,
    PostId,
    UserId,
    ParentId,
    #[sea_orm(iden = "text")]
    Comment,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum PostLikes {
    Table,
    Id,
    PostId,
    UserId,
}