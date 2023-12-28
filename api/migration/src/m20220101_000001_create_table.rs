use sea_orm_migration::prelude::*;
use crate::tables::{Bookmarks, Favorites, Followers, Following, PostComments, PostFiles, PostLikes, Posts, Stories, Users};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        // Start with the bottom top level table to avoid foreign key errors

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string_len(65).not_null())
                    .col(ColumnDef::new(Users::Username).string_len(32).not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string_len(32).not_null().unique_key())
                    .col(ColumnDef::new(Users::Password).text().not_null())
                    .col(ColumnDef::new(Users::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Users::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Followers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Followers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Followers::UserId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Followers::FollowerId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Followers::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Following::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Following::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Following::UserId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Following::FollowingId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(Following::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Stories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Stories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Stories::UserId).uuid().not_null())
                    .col(ColumnDef::new(Stories::FileUrl).text().not_null())
                    .col(ColumnDef::new(Stories::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stories_users")
                            .from(Stories::Table, Stories::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::UserId).uuid().not_null())
                    .col(ColumnDef::new(Posts::Description).text().not_null())
                    .col(ColumnDef::new(Posts::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Posts::UpdatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_users")
                            .from(Posts::Table, Posts::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Bookmarks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bookmarks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bookmarks::UserId).uuid().not_null())
                    .col(ColumnDef::new(Bookmarks::PostId).uuid().not_null())
                    .col(ColumnDef::new(Bookmarks::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookmarks_users")
                            .from(Bookmarks::Table, Bookmarks::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_bookmarks_posts")
                            .from(Bookmarks::Table, Bookmarks::PostId)
                            .to(Posts::Table, Posts::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Favorites::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Favorites::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Favorites::UserId).uuid().not_null())
                    .col(ColumnDef::new(Favorites::PostId).uuid().not_null())
                    .col(ColumnDef::new(Favorites::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favorites_users")
                            .from(Favorites::Table, Favorites::UserId)
                            .to(Users::Table, Users::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favorites_posts")
                            .from(Favorites::Table, Favorites::PostId)
                            .to(Posts::Table, Posts::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostLikes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostLikes::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostLikes::UserId).uuid().not_null().unique_key())
                    .col(ColumnDef::new(PostLikes::PostId).uuid().not_null().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_likes_posts")
                            .from(PostLikes::Table, PostLikes::PostId)
                            .to(Posts::Table, Posts::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostFiles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostFiles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostFiles::PostId).uuid().not_null())
                    .col(ColumnDef::new(PostFiles::FileUrl).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_files_posts")
                            .from(PostLikes::Table, PostLikes::PostId)
                            .to(Posts::Table, Posts::Id)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostComments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostComments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostComments::UserId).uuid().not_null())
                    .col(ColumnDef::new(PostComments::PostId).uuid().not_null())
                    .col(ColumnDef::new(PostComments::ParentId).uuid())
                    .col(ColumnDef::new(PostComments::Comment).text().not_null())
                    .col(ColumnDef::new(PostComments::CreatedAt).timestamp().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_comments_posts")
                            .from(PostLikes::Table, PostLikes::PostId)
                            .to(Posts::Table, Posts::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Users::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Followers::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Following::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Stories::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Bookmarks::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Favorites::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(Posts::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PostFiles::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PostLikes::Table)
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .if_exists()
                    .table(PostComments::Table)
                    .to_owned()
            )
            .await
    }
}