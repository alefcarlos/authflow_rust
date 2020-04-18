use super::{Field, ISORequest};
use crate::domain::{AuthorizerError, AuthorizerResult, TransactionType, MESSAGE_TYPE_INDICATOR, RESPONSE_CODE};

pub struct ISOResponsePrepareParams {
    pub request: ISORequest,
    pub transaction: TransactionType,
    pub authorizer_result: Result<AuthorizerResult, AuthorizerError>,
}

pub struct ISOResponse {
    fields: Vec<Field>,
}

impl ISOResponse {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }

    fn add_field(&mut self, value: Field) {
        self.fields.push(value);
    }

    fn rm_field(&mut self, id: &str) {
        let index = self.fields.iter().position(|f| f.id == id);

        match index {
            Some(v) => self.fields.remove(v),
            None => return (),
        };
    }

    ///Gets value from DE
    pub fn get_info(&self, id: &str) -> Option<String> {
        let item = self.fields.iter().find(|&field| field.id == id);

        match item {
            Some(x) => Some(x.value.clone()),
            None => None,
        }
    }
}

impl From<ISOResponsePrepareParams> for ISOResponse {
    fn from(value: ISOResponsePrepareParams) -> Self {
        let mut response = Self::from(value.request);

        // Delete default DE
        response.rm_field(MESSAGE_TYPE_INDICATOR);

        //TODO: remover DE de acordo com a transaction
        // match value.transaction {
        //     TransactionType::OlinePurchase(_) => {
        //         response.rm_field("1");
        //         response.rm_field("55");
        //     },
        //     _ => (),
        // }

        //TODO: aplicar novos DE
        response.add_field(Field {
            id: RESPONSE_CODE.to_owned(),
            value: "00".to_owned(),
        });

        response
    }
}

impl From<ISORequest> for ISOResponse {
    fn from(value: ISORequest) -> Self {
        let mut this = Self::new();

        for f in value.fields.iter() {
            this.add_field(Field {
                id: f.id.clone(),
                value: f.value.clone(),
            });
        }

        this
    }
}
