use quasar_idl::lint;
use quasar_idl::lint::graph::AccountGraph;
use quasar_idl::parser;

#[test]
fn lint_report_empty_for_constrained_program() {
    let src = r#"
        declare_id!("11111111111111111111111111111111");

        #[program]
        mod test_program {
            use super::*;

            #[instruction(discriminator = [1])]
            pub fn approve(ctx: Ctx<Approve>) -> Result<(), ProgramError> {
                Ok(())
            }
        }

        #[derive(Accounts)]
        pub struct Approve<'info> {
            pub authority: Signer,
            #[account(mut, has_one = authority)]
            pub vault: Account<Vault<'info>>,
        }

        #[account(discriminator = 1)]
        pub struct Vault {
            pub authority: Address,
            pub balance: u64,
        }
    "#;

    let parsed = quasar_idl::parser::parse_program_from_source(src);
    let report = lint::run_lint(&parsed, &lint::LintConfig::default());
    assert!(
        report.diagnostics.is_empty(),
        "expected no diagnostics, got: {:?}",
        report.diagnostics
    );
}

#[test]
fn parses_has_one_constraints() {
    let src = r#"
        declare_id!("11111111111111111111111111111111");

        #[program]
        mod test_program {
            use super::*;
            #[instruction(discriminator = [1])]
            pub fn approve(ctx: Ctx<Approve>) -> Result<(), ProgramError> {
                Ok(())
            }
        }

        #[derive(Accounts)]
        pub struct Approve<'info> {
            pub wallet: Account<Wallet<'info>>,
            pub intent: Account<Intent<'info>>,
            #[account(mut, has_one = wallet, has_one = intent)]
            pub proposal: Account<Proposal<'info>>,
        }

        #[account(discriminator = 1)]
        pub struct Proposal {
            pub wallet: Address,
            pub intent: Address,
        }

        #[account(discriminator = 2)]
        pub struct Wallet {
            pub name: u64,
        }

        #[account(discriminator = 3)]
        pub struct Intent {
            pub threshold: u8,
        }
    "#;

    let parsed = parser::parse_program_from_source(src);
    let proposal_field = parsed.accounts_structs[0]
        .fields
        .iter()
        .find(|f| f.name == "proposal")
        .unwrap();

    assert_eq!(proposal_field.constraints.has_ones, vec!["wallet", "intent"]);
    assert!(proposal_field.constraints.is_mut);
}

#[test]
fn graph_has_correct_edges_for_has_one() {
    let src = r#"
        declare_id!("11111111111111111111111111111111");
        #[program]
        mod test_program {
            use super::*;
            #[instruction(discriminator = [1])]
            pub fn approve(ctx: Ctx<Approve>) -> Result<(), ProgramError> { Ok(()) }
        }
        #[derive(Accounts)]
        pub struct Approve<'info> {
            pub authority: Signer,
            pub wallet: Account<Wallet<'info>>,
            #[account(mut, has_one = wallet)]
            pub proposal: Account<Proposal<'info>>,
        }
        #[account(discriminator = 1)]
        pub struct Proposal { pub wallet: Address }
        #[account(discriminator = 2)]
        pub struct Wallet { pub bump: u8 }
    "#;
    let parsed = quasar_idl::parser::parse_program_from_source(src);
    let registry = quasar_idl::lint::types::TypeRegistry::from_parsed(&parsed);
    let graph = AccountGraph::build(&parsed.accounts_structs[0], &registry);

    assert_eq!(graph.nodes.len(), 3);
    assert!(graph.has_edge("proposal", "wallet"));
}
