// @generated SignedSource<<d7341dfcdb893900eeaa099faf47f0c4>>

mod refetchable_fragment;

use refetchable_fragment::transform_fixture;
use fixture_tests::test_fixture;

#[test]
fn fragment_on_interface_which_implmentations_implement_node() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-implement-node.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-implement-node.expected");
    test_fixture(transform_fixture, "fragment-on-interface-which-implmentations-implement-node.graphql", "refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-implement-node.expected", input, expected);
}

#[test]
fn fragment_on_interface_which_implmentations_not_implement_node_invalid() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-not-implement-node.invalid.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-not-implement-node.invalid.expected");
    test_fixture(transform_fixture, "fragment-on-interface-which-implmentations-not-implement-node.invalid.graphql", "refetchable_fragment/fixtures/fragment-on-interface-which-implmentations-not-implement-node.invalid.expected", input, expected);
}

#[test]
fn fragment_on_node_interface() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-node-interface.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-node-interface.expected");
    test_fixture(transform_fixture, "fragment-on-node-interface.graphql", "refetchable_fragment/fixtures/fragment-on-node-interface.expected", input, expected);
}

#[test]
fn fragment_on_node_interface_without_id() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-node-interface-without-id.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-node-interface-without-id.expected");
    test_fixture(transform_fixture, "fragment-on-node-interface-without-id.graphql", "refetchable_fragment/fixtures/fragment-on-node-interface-without-id.expected", input, expected);
}

#[test]
fn fragment_on_object_implementing_node_interface() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface.expected");
    test_fixture(transform_fixture, "fragment-on-object-implementing-node-interface.graphql", "refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface.expected", input, expected);
}

#[test]
fn fragment_on_object_implementing_node_interface_with_alias_id() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface-with-alias-id.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface-with-alias-id.expected");
    test_fixture(transform_fixture, "fragment-on-object-implementing-node-interface-with-alias-id.graphql", "refetchable_fragment/fixtures/fragment-on-object-implementing-node-interface-with-alias-id.expected", input, expected);
}

#[test]
fn fragment_on_query() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-query.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-query.expected");
    test_fixture(transform_fixture, "fragment-on-query.graphql", "refetchable_fragment/fixtures/fragment-on-query.expected", input, expected);
}

#[test]
fn fragment_on_query_with_cycle() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-query-with-cycle.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-query-with-cycle.expected");
    test_fixture(transform_fixture, "fragment-on-query-with-cycle.graphql", "refetchable_fragment/fixtures/fragment-on-query-with-cycle.expected", input, expected);
}

#[test]
fn fragment_on_query_without_query_name_invalid() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-query-without-query-name.invalid.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-query-without-query-name.invalid.expected");
    test_fixture(transform_fixture, "fragment-on-query-without-query-name.invalid.graphql", "refetchable_fragment/fixtures/fragment-on-query-without-query-name.invalid.expected", input, expected);
}

#[test]
fn fragment_on_viewer() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-on-viewer.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-on-viewer.expected");
    test_fixture(transform_fixture, "fragment-on-viewer.graphql", "refetchable_fragment/fixtures/fragment-on-viewer.expected", input, expected);
}

#[test]
fn fragment_with_args_on_object_implementing_node_interface() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-object-implementing-node-interface.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-object-implementing-node-interface.expected");
    test_fixture(transform_fixture, "fragment-with-args-on-object-implementing-node-interface.graphql", "refetchable_fragment/fixtures/fragment-with-args-on-object-implementing-node-interface.expected", input, expected);
}

#[test]
fn fragment_with_args_on_query() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-query.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-query.expected");
    test_fixture(transform_fixture, "fragment-with-args-on-query.graphql", "refetchable_fragment/fixtures/fragment-with-args-on-query.expected", input, expected);
}

#[test]
fn fragment_with_args_on_viewer() {
    let input = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-viewer.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/fragment-with-args-on-viewer.expected");
    test_fixture(transform_fixture, "fragment-with-args-on-viewer.graphql", "refetchable_fragment/fixtures/fragment-with-args-on-viewer.expected", input, expected);
}

#[test]
fn refetchable_fragment_with_connection() {
    let input = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection.expected");
    test_fixture(transform_fixture, "refetchable-fragment-with-connection.graphql", "refetchable_fragment/fixtures/refetchable-fragment-with-connection.expected", input, expected);
}

#[test]
fn refetchable_fragment_with_connection_bidirectional() {
    let input = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection-bidirectional.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection-bidirectional.expected");
    test_fixture(transform_fixture, "refetchable-fragment-with-connection-bidirectional.graphql", "refetchable_fragment/fixtures/refetchable-fragment-with-connection-bidirectional.expected", input, expected);
}

#[test]
fn refetchable_fragment_with_connection_with_stream() {
    let input = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection-with-stream.graphql");
    let expected = include_str!("refetchable_fragment/fixtures/refetchable-fragment-with-connection-with-stream.expected");
    test_fixture(transform_fixture, "refetchable-fragment-with-connection-with-stream.graphql", "refetchable_fragment/fixtures/refetchable-fragment-with-connection-with-stream.expected", input, expected);
}
