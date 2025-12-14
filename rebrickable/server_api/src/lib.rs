pub mod query {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use clap::Subcommand;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum PartGetType {
        /// Get the part by its id
        Id { id: PartId },
        /// Get the part by its name
        Name { name: PartName },
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum ColorGetType {
        /// Get the color by its id
        Id { id: ColorId },
        /// Get the color by its name
        Name { name: ColorName },
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

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum PartFindType {
        /// Find the part by its id
        Id,
        /// Find the part by its name
        Name,
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
    pub enum ColorFindType {
        /// Find the color by its id
        Id,
        /// Find the color by its name
        Name,
    }

    #[derive(Debug, Clone, Subcommand, Decode, Encode, Deserialize, Serialize)]
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

    #[derive(Subcommand, Debug, Clone, Decode, Encode, Deserialize, Serialize)]
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
}

pub mod response {
    use rebrickable_database_api::*;

    use bincode::{Decode, Encode};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum GetItemResponse {
        Part(Part),
        Color(Color),
        Element(Element),
        NotFound,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum IterItemsResponse {
        PartId(PartId),
        PartName(PartName),
        ColorId(ColorId),
        ColorName(ColorName),
        ElementId(ElementId),
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
    pub enum Response {
        GetItem(GetItemResponse, crate::query::GetItem),
        /// The Option is None once the stream has ended.
        IterItems(Option<IterItemsResponse>),
    }
}
