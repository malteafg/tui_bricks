use rebrickable_database_api::{ColorId, ColorName, ElementId, PartId, PartName};

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum PartGetType {
    /// Get the part by its id
    Id { id: PartId },
    /// Get the part by its name
    Name { name: PartName },
}

#[derive(Debug, Clone, Subcommand)]
pub enum ColorGetType {
    /// Get the color by its id
    Id { id: ColorId },
    /// Get the color by its name
    Name { name: ColorName },
}

#[derive(Debug, Clone, Subcommand)]
pub enum GetItem {
    Part {
        #[command(subcommand)]
        part: PartGetType,
    },
    Color {
        #[command(subcommand)]
        color: ColorGetType,
    },
    Element {
        id: ElementId,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum PartFindType {
    /// Find the part by its id
    Id,
    /// Find the part by its name
    Name,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ColorFindType {
    /// Find the color by its id
    Id,
    /// Find the color by its name
    Name,
}

#[derive(Debug, Clone, Subcommand)]
pub enum FindItem {
    Part {
        #[command(subcommand)]
        part: PartFindType,
    },
    Color {
        #[command(subcommand)]
        color: ColorFindType,
    },
    Element,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Query {
    Get {
        /// The type of item to get. This can be a part, color or element.
        #[command(subcommand)]
        get_item: GetItem,
    },
    Find {
        /// The type of item to find. This can be a part, color or element.
        #[command(subcommand)]
        find_item: FindItem,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    pub query: Query,
}
