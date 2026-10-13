use super::*;

/// `kind_label_for`/`channel_label_for` end in `_ => None`, so there is no
/// type for the compiler to be exhaustive over at this seam — adding a
/// notification kind or channel in `peisear-core` without updating the map
/// here would ship a raw id to a user's inbox with no compile error, no
/// guard rejection. This test converts that hole into a failing test the
/// moment a kind/channel is declared without a label.
#[test]
fn every_declared_notification_kind_has_a_label() {
    for kind_id in peisear_core::notifications::kind::all_user_facing() {
        assert!(
            kind_label_for(kind_id).is_some(),
            "notifications::kind::{kind_id:?} has no NotificationKindLabel mapping in \
             kind_label_for — add one so it doesn't ship its raw id to a user's inbox"
        );
    }
}

#[test]
fn every_declared_notification_channel_has_a_label() {
    for channel_id in peisear_core::notifications::channel::all() {
        assert!(
            channel_label_for(channel_id).is_some(),
            "notifications::channel::{channel_id:?} has no NotificationChannelLabel \
             mapping in channel_label_for — add one so it doesn't ship its raw id"
        );
    }
}
