extern crate proc_macro;
use proc_macro::TokenStream;

/**
 * @brief      A procedural macro that reads a .dna.json file and builds a struct definition
 *             which can be used to make strongly(ish) typed calls to a conductor running the dna
 */
#[proc_macro]
pub fn client_from_dna(_item: TokenStream) -> TokenStream {
    
}
