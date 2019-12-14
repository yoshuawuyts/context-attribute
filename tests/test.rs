use context_attribute::context;
use failure::{self, ResultExt};

use failure::Error;

/// doc of implicit_doc_context
#[context]
fn implicit_doc_context() -> Result<String, Error> {
    return Err(failure::err_msg("xxxx"));
}

/// doc of explicit_doc_context
#[context(doc)]
fn explicit_doc_context() -> Result<String, Error> {
    return Err(failure::err_msg("xxxx"));
}

#[context(fn)]
fn explicit_fn_name_context() -> Result<String, Error> {
    return Err(failure::err_msg("xxxx"));
}

#[context(msg:"custom msg")]
fn explicit_custom_context() -> Result<String, Error> {
    return Err(failure::err_msg("xxxx"));
}

fn assert_err_contains<T>(res: Result<T, failure::Error>, msg: &str) {
    if let Err(e) = res {
        assert!(e.to_string().contains(msg));
    }
    assert!(true);
}

#[test]
fn test_context() -> Result<(), Error> {
    assert_err_contains(implicit_doc_context(), "doc of implicit_doc_context");
    assert_err_contains(explicit_doc_context(), "doc of explicit_doc_context");
    assert_err_contains(explicit_fn_name_context(), "explicit_fn_name_context");
    assert_err_contains(explicit_custom_context(), "custom msg");

    // assert!(false);
    Ok(())
}
