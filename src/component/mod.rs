use crate::style::ComponentStyle;
use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChatComponent {
    #[cfg_attr(feature = "serde", serde(flatten))]
    kind: ComponentType,
    #[cfg_attr(feature = "serde", serde(flatten))]
    style: ComponentStyle,
    #[cfg_attr(
        feature = "serde",
        serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)
    )]
    siblings: Vec<ChatComponent>,
}

impl ChatComponent {
    pub fn from_component(kind: ComponentType, style: ComponentStyle) -> Self {
        ChatComponent {
            kind,
            style,
            siblings: vec![],
        }
    }

    pub fn from_text<T: Into<String>>(text: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Text(TextComponent::from_text(text)),
            style,
            siblings: vec![],
        }
    }

    pub fn from_key<T: Into<String>>(key: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Translation(TranslationComponent::from_key(key)),
            style,
            siblings: vec![],
        }
    }

    pub fn from_score<T: Into<String>, U: Into<String>>(
        name: T,
        objective: U,
        style: ComponentStyle,
    ) -> Self {
        ChatComponent {
            kind: ComponentType::Score(ScoreComponent::from_score(name, objective)),
            style,
            siblings: vec![],
        }
    }

    pub fn from_selector<T: Into<String>>(selector: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Selector(SelectorComponent::from_selector(selector)),
            style,
            siblings: vec![],
        }
    }

    pub fn from_keybind<T: Into<String>>(keybind: T, style: ComponentStyle) -> Self {
        ChatComponent {
            kind: ComponentType::Keybind(KeybindComponent::from_keybind(keybind)),
            style,
            siblings: vec![],
        }
    }

    pub fn get_kind(&self) -> &ComponentType {
        &self.kind
    }

    pub fn get_kind_mut(&mut self) -> &mut ComponentType {
        &mut self.kind
    }

    pub fn get_style(&self) -> &ComponentStyle {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut ComponentStyle {
        &mut self.style
    }

    pub fn get_siblings(&self) -> &Vec<ChatComponent> {
        &self.siblings
    }

    pub fn get_siblings_mut(&mut self) -> &mut Vec<ChatComponent> {
        &mut self.siblings
    }
}

impl Deref for ChatComponent {
    type Target = ComponentStyle;

    fn deref(&self) -> &Self::Target {
        &self.style
    }
}

impl DerefMut for ChatComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.style
    }
}

/// The different kinds of components Minecraft chat messages
/// can be made up of. One component (`storage`-component, since 1.15) is missing,
/// further research and contributions on this would be appreciated!
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ComponentType {
    Text(TextComponent),
    Translation(TranslationComponent),
    /// # Warning
    /// Since **1.8**!
    ///
    /// This crate does not check any version,
    /// it is up to the user to deal with this safely!
    Score(ScoreComponent),
    /// # Warning
    /// Since **1.8** and **Client-To-Server** only!
    ///
    /// This crate does not check these constraints,
    /// it is up to the user to deal with this safely!
    Selector(SelectorComponent),
    /// # Warning
    /// Since **1.12**!
    ///
    /// This crate does not check any version,
    /// it is up to the user to deal with this safely!
    Keybind(KeybindComponent),
    // TODO: research the `storage` component (since 1.15)
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextComponent {
    text: String,
}

impl TextComponent {
    pub fn from_text<T: Into<String>>(text: T) -> Self {
        TextComponent { text: text.into() }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into()
    }

    pub fn text<T: Into<String>>(mut self, text: T) -> Self {
        self.set_text(text);
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TranslationComponent {
    #[cfg_attr(feature = "serde", serde(rename = "translate"))]
    key: String,
    with: Vec<ChatComponent>,
}

impl TranslationComponent {
    pub fn from_key<T: Into<String>>(key: T) -> Self {
        TranslationComponent {
            key: key.into(),
            with: vec![],
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn set_key<T: Into<String>>(&mut self, key: T) {
        self.key = key.into()
    }

    pub fn key<T: Into<String>>(mut self, key: T) -> Self {
        self.set_key(key);
        self
    }

    pub fn add_arg(&mut self, component: ChatComponent) {
        self.with.push(component)
    }

    pub fn argument(mut self, component: ChatComponent) -> Self {
        self.add_arg(component);
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoreComponent {
    name: String,
    objective: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    value: Option<String>,
}

impl ScoreComponent {
    pub fn from_score<T: Into<String>, U: Into<String>>(name: T, objective: U) -> Self {
        ScoreComponent {
            name: name.into(),
            objective: objective.into(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name<T: Into<String>>(&mut self, name: T) {
        self.name = name.into()
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    pub fn get_objective(&self) -> &String {
        &self.objective
    }

    pub fn set_objective<T: Into<String>>(&mut self, objective: T) {
        self.objective = objective.into()
    }

    pub fn objective<T: Into<String>>(mut self, objective: T) -> Self {
        self.set_objective(objective);
        self
    }

    pub fn get_value(&self) -> Option<&String> {
        self.value.as_ref()
    }

    pub fn set_value<T: Into<String>>(&mut self, value: Option<T>) {
        self.value = value.map(|value| value.into());
    }

    pub fn value<T: Into<String>>(mut self, value: Option<T>) -> Self {
        self.set_value(value);
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SelectorComponent {
    selector: String,
}

impl SelectorComponent {
    pub fn from_selector<T: Into<String>>(selector: T) -> Self {
        SelectorComponent {
            selector: selector.into(),
        }
    }

    pub fn get_selector(&self) -> &String {
        &self.selector
    }

    pub fn set_selector<T: Into<String>>(&mut self, selector: T) {
        self.selector = selector.into()
    }

    pub fn selector<T: Into<String>>(mut self, selector: T) -> Self {
        self.set_selector(selector);
        self
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeybindComponent {
    keybind: String,
}

impl KeybindComponent {
    pub fn from_keybind<T: Into<String>>(keybind: T) -> Self {
        KeybindComponent {
            keybind: keybind.into(),
        }
    }

    pub fn get_keybind(&self) -> &String {
        &self.keybind
    }

    pub fn set_keybind<T: Into<String>>(&mut self, keybind: T) {
        self.keybind = keybind.into()
    }

    pub fn keybind<T: Into<String>>(mut self, keybind: T) -> Self {
        self.set_keybind(keybind);
        self
    }
}
