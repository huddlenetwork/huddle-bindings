#[cfg(test)]
mod test {
    use crate::chain_communication::DesmosCli;
    use crate::consts::{
        TEST_DELETABLE_ATTACHMENT_ID, TEST_POLL_ID, TEST_SUBSPACE, TEST_SUBSPACE_DELETABLE_POST_ID,
        TEST_SUBSPACE_EDITABLE_POST_ID,
    };
    use cosmwasm_std::{Addr, Uint64};
    use desmos_bindings::posts::models::{
        Entities, PostAttachment, PostReference, PostReferenceType, ProvidedAnswer, ReplySetting,
        UrlEntity,
    };
    use desmos_bindings::posts::msg::PostsMsg;
    use test_contract::msg::ExecuteMsg;

    #[test]
    fn test_create_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::CreatePost {
            subspace_id: TEST_SUBSPACE,
            section_id: 0,
            external_id: None,
            text: Some("Post text".to_string()),
            entities: None,
            tags: vec![],
            attachments: None,
            author: Addr::unchecked(&contract_address),
            conversation_id: None,
            reply_settings: ReplySetting::Everyone,
            referenced_posts: vec![PostReference {
                post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
                ref_type: PostReferenceType::Replay,
                position: None,
            }],
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_edit_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::EditPost {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            text: "[do-not-modify]".to_string(),
            entities: Some(Entities {
                hashtags: vec![],
                mentions: vec![],
                urls: vec![UrlEntity {
                    start: Uint64::new(0),
                    end: Uint64::new(1),
                    url:
                        "https://ipfs.infura.io/ipfs/QmT3AenKHkhCeesTUdnarqUVu91mmBk1cxQknxnUd79gY7"
                            .into(),
                    display_url: "IPFS".into(),
                }],
            }),
            editor: Addr::unchecked(&contract_address),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_delete_post() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::DeletePost {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_DELETABLE_POST_ID,
            signer: Addr::unchecked(&contract_address),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_add_media_post_attachment() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg_add_media = PostsMsg::AddPostAttachment {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            content: PostAttachment::Media {
                mime_type: "test-mime".to_string(),
                uri: "https://test.com/image.png".to_string(),
            }
            .into(),
            editor: Addr::unchecked(&contract_address),
        };

        let msg_add_poll = PostsMsg::AddPostAttachment {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            content: PostAttachment::Poll {
                question: "Test question?".to_string(),
                provided_answers: vec![
                    ProvidedAnswer {
                        text: Some("Answer 1".to_string()),
                        attachments: vec![],
                    },
                    ProvidedAnswer {
                        text: Some("Answer 2".to_string()),
                        attachments: vec![],
                    },
                ],
                end_date: "2140-01-01T10:00:20.021Z".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
                final_tally_results: None,
            }
            .into(),
            editor: Addr::unchecked(&contract_address),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg_add_media.into(), msg_add_poll.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_remove_post_attachment() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::RemovePostAttachment {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            attachment_id: TEST_DELETABLE_ATTACHMENT_ID,
            editor: Addr::unchecked(&contract_address),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }

    #[test]
    fn test_answer_poll() {
        let desmos_cli = DesmosCli::default();
        let contract_address = desmos_cli.get_contract_by_code(1);

        let msg = PostsMsg::AnswerPoll {
            subspace_id: TEST_SUBSPACE,
            post_id: TEST_SUBSPACE_EDITABLE_POST_ID,
            poll_id: TEST_POLL_ID,
            answers_indexes: vec![0],
            signer: Addr::unchecked(&contract_address),
        };

        desmos_cli
            .wasm_execute(
                &contract_address,
                &ExecuteMsg::DesmosMessages {
                    msgs: vec![msg.into()],
                },
            )
            .assert_success();
    }
}
