fn parent_method() {}

mod parent_mod {
    fn child_method() {
        super::parent_method();
    }
}

