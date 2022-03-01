use dataloaders::{Batcher, Loader};
use objects::{
    auction_house::AuctionHouse,
    listing::{Bid, Listing},
    nft::{Nft, NftAttribute, NftCreator, NftOwner},
    storefront::Storefront,
};
use strings::{AuctionHouseAddress, ListingAddress, MetadataAddress, StorefrontAddress};

use super::prelude::*;

#[derive(Clone)]
pub struct AppContext {
    pub db_pool: Arc<Pool>,
    pub twitter_bearer_token: Arc<String>,

    // Data loaders
    pub auction_house_loader: Loader<AuctionHouseAddress, Option<AuctionHouse>>,
    pub listing_loader: Loader<ListingAddress, Option<Listing>>,
    pub listing_bids_loader: Loader<ListingAddress, Vec<Bid>>,
    pub listing_nfts_loader: Loader<ListingAddress, Vec<Nft>>,
    pub nft_attributes_loader: Loader<MetadataAddress, Vec<NftAttribute>>,
    pub nft_creators_loader: Loader<MetadataAddress, Vec<NftCreator>>,
    pub nft_owner_loader: Loader<MetadataAddress, Option<NftOwner>>,
    pub storefront_loader: Loader<StorefrontAddress, Option<Storefront>>,
}

impl juniper::Context for AppContext {}

impl AppContext {
    pub fn new(db_pool: Arc<Pool>, twitter_bearer_token: Arc<String>) -> AppContext {
        let batcher = Batcher::new(db_pool.clone());

        Self {
            auction_house_loader: Loader::new(batcher.clone()),
            listing_loader: Loader::new(batcher.clone()),
            listing_bids_loader: Loader::new(batcher.clone()),
            listing_nfts_loader: Loader::new(batcher.clone()),
            nft_attributes_loader: Loader::new(batcher.clone()),
            nft_creators_loader: Loader::new(batcher.clone()),
            nft_owner_loader: Loader::new(batcher.clone()),
            storefront_loader: Loader::new(batcher),

            db_pool,
            twitter_bearer_token,
        }
    }
}