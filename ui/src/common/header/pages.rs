use router::RootRoutes;

pub struct Page {
    pub route: RootRoutes,
    pub name: &'static str,
}

pub const PAGES: [Page; 4] = [
    Page {
        route: RootRoutes::Home,
        name: "Posts",
    },
    Page {
        route: RootRoutes::Projects,
        name: "Projects",
    },
    Page {
        route: RootRoutes::About,
        name: "About",
    },
    Page {
        route: RootRoutes::Links,
        name: "Links",
    },
];
