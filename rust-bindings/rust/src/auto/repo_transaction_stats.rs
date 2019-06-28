// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gobject_sys;
use ostree_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RepoTransactionStats(Boxed<ostree_sys::OstreeRepoTransactionStats>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(ostree_sys::ostree_repo_transaction_stats_get_type(), ptr as *mut _) as *mut ostree_sys::OstreeRepoTransactionStats,
        free => |ptr| gobject_sys::g_boxed_free(ostree_sys::ostree_repo_transaction_stats_get_type(), ptr as *mut _),
        init => |_ptr| (),
        clear => |_ptr| (),
        get_type => || ostree_sys::ostree_repo_transaction_stats_get_type(),
    }
}
