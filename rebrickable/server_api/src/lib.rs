pub mod query {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use clap::{Subcommand, ValueEnum};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, ValueEnum, Decode, Encode, Deserialize, Serialize)]
    pub enum ItemType {
        Part,
        Color,
        Element,
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum PartGetType {
        /// Get the part by its id
        Id { id: PartId },
        /// Get the part by its name
        Name { name: String },
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum ColorGetType {
        /// Get the part by its id
        Id { id: ColorId },
        /// Get the part by its name
        Name { name: String },
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
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

    #[derive(Subcommand, Debug, Clone, Decode, Encode, Deserialize, Serialize)]
    pub enum Query {
        Get {
            #[command(subcommand)]
            get_item: GetItem,
        },
        Find {
            /// The type of item to find. This can be a part, color or element.
            item_type: ItemType,
        },
    }
}

pub mod response {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum GetItemResponse {
        Part(Part),
        Color(ColorRecord),
        Element(ElementRecord),
        NotFound,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum IterItemsResponse {
        PartId(PartId),
        ColorId(ColorId),
        ElementId(ElementId),
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum Response {
        GetItem(GetItemResponse, crate::query::GetItem),
        /// The Option is None once the stream has ended.
        IterItems(Option<IterItemsResponse>),
    }
}
