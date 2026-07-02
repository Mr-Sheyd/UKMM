#[derive(Debug, Clone)]
pub enum ContextData {
    Parameter(roead::aamp::Parameter),
    List(roead::aamp::ParameterList),
    Object(roead::aamp::ParameterObject),
    Byml(roead::byml::Byml),
}

impl From<roead::aamp::Parameter> for ContextData {
    fn from(param: roead::aamp::Parameter) -> Self {
        ContextData::Parameter(param)
    }
}

impl From<&roead::aamp::Parameter> for ContextData {
    fn from(param: &roead::aamp::Parameter) -> Self {
        ContextData::Parameter(param.clone())
    }
}

impl From<roead::aamp::ParameterList> for ContextData {
    fn from(list: roead::aamp::ParameterList) -> Self {
        ContextData::List(list)
    }
}

impl From<roead::aamp::ParameterObject> for ContextData {
    fn from(obj: roead::aamp::ParameterObject) -> Self {
        ContextData::Object(obj)
    }
}

impl From<&roead::aamp::ParameterList> for ContextData {
    fn from(list: &roead::aamp::ParameterList) -> Self {
        ContextData::List(list.clone())
    }
}

impl From<&roead::aamp::ParameterObject> for ContextData {
    fn from(obj: &roead::aamp::ParameterObject) -> Self {
        ContextData::Object(obj.clone())
    }
}

impl From<roead::byml::Byml> for ContextData {
    fn from(by: roead::byml::Byml) -> Self {
        ContextData::Byml(by)
    }
}

impl From<&roead::byml::Byml> for ContextData {
    fn from(by: &roead::byml::Byml) -> Self {
        ContextData::Byml(by.clone())
    }
}