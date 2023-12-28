use handlebars::{Context, Handlebars, RenderError};

use lazy_static::lazy_static;

use crate::ModLinksChangelog;

const MARKDOWN_TEMPLATE_NAME: &str = "markdown";
pub const CHANGELOG_TEMPLATE_MARKDOWN: &str = include_str!("./../assets/changelog-template.md");

lazy_static! {
    static ref HANDLEBARS_INSTANCE: Handlebars<'static> = {
        let mut instance = Handlebars::new();
        instance.set_strict_mode(true);

        #[cfg(debug_assertions)]
        instance.set_dev_mode(true);

        instance
            .register_template_string(MARKDOWN_TEMPLATE_NAME, CHANGELOG_TEMPLATE_MARKDOWN)
            .unwrap();

        instance
    };
}

impl ModLinksChangelog {
    #[inline]
    pub fn to_markdown(&self) -> Result<String, RenderError> {
        HANDLEBARS_INSTANCE.render(MARKDOWN_TEMPLATE_NAME, self.json())
    }
}

impl From<ModLinksChangelog> for Context {
    #[inline]
    fn from(value: ModLinksChangelog) -> Self {
        value.ctx.into()
    }
}
