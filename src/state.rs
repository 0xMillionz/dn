use anchor_client::{
	Client,
	Cluster,
	EventContext,
	solana_sdk::signature::{
		Keypair,
		Signer,
		read_keypair_file,
	},
};
use std::{
	rc::Rc,
	fs,
	sync::Arc,
};
use arc_swap::ArcSwap;

pub struct DnState{
	pub mango_client: ArcSwap<Arc<Client>>,
	pub payer: dyn Signer,
	// idk add other stuff in here too XD
}

// meh this will probably get bigger
impl DnState{
	pub fn new() -> &'static DnState {
		// Config Settings
		let url = Cluster::Mainnet;
		let payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/payer.json"))
			.expect("Wya wyd keypair???");

		let client = Client::new_with_options(cluster: Cluster, Rc::new(payer), options: CommitmentConfig)

		let dn_state = DnState {
			mango_client: ArcSwap::from_pointee(Arc::new()),
		};

		Box::leak(Box::new(dn_state))
	}
}