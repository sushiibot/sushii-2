use twilight::model::guild::Permissions as TwilightPermissions;

pub struct Permissions(TwilightPermissions);

impl Permissions {
    pub fn new() -> Self {
        Permissions(TwilightPermissions::empty())
    }

    pub fn from_permission(permission: TwilightPermissions) -> Self {
        Permissions::new().add_permission(permission)
    }

    pub fn add_permission(mut self, permission: TwilightPermissions) -> Self {
        self.0.insert(permission);
        self
    }

    pub fn build(self) -> TwilightPermissions {
        self.0
    }
}
