

enum ProtocolOP
{
    bindRequest = 0,
    bindResponse,
    unbindRequest,
    searchRequest,
    searchResEntry,
    searchResDone,
    searchResRef,
    modifyRequest,
    modifyResponse,
    addRequest,
    addResponse,
    delRequest,
    delResponse,
    modDNRequest,
    modDNResponse,
    compareRequest,
    compareResponse,
    abandonRequest,
    extendedReq,
    extendedResp,
    intermediateResponse,
}

enum resultCode
{
    success = 0,
    operationError = 1,
    protocolError = 2,
    timeLimitExceeded = 3,
    sizeLimitExceeded = 4,
    compareFalse = 5,
    compareTrue = 6,
    authMethodNotSupported = 7,
    strongerAuthRequired = 8,
    /* 9 reserved */
    referral = 10,
    adminLimitExceeded = 11,
    unavailableCriticalExtension = 12,
    confidentialityRequired = 13,
    saslBindInProgress = 14,
    /* 15 undefined */
    noSuchAttribute = 16,
    undefinedAttributeType = 17,
    inappropriateMatching = 18,
    constraintViolation = 19,
    attributeOrValueExists = 20,
    invalidAttributeSyntax = 21,
    /* 22-31 unused */
    noSuchObject = 32,
    aliasProblem = 33,
    invalidDNSyntax = 34,
    /* 35 reserved for undefined isLeaf */
    aliasDereferencingProblem = 36,
    /* 37-47 unused */
    inappropriateAuthentication = 48,
    invalidCredentials = 49,
    insufficientAccessRights = 50,
    busy = 51,
    unavailable = 52,
    unwillingToPerform = 53,
    loopDetect = 54,
    /* 55-63 unused */
    namingViolation = 64,
    objectClassViolation = 65,
    notAllowedOnNonLeaf = 66,
    notAllowedOnRDN = 67,
    entryAlreadyExists = 68,
    objectClassModsProhibited = 69,
    /* 70 reserved for CLDAP */
    affectsMultipleDSAs = 71,
    /* 72-79 unused */
    other = 80,
}
// impl resultCode
// {
//     pub fn from_u8(v: u8) -> resultCode
//     {
//         match v
//         {
//    // success = 0,
//     // operationError = 1,
//     // protocolError = 2,
//     // timeLimitExceeded = 3,
//     // sizeLimitExceeded = 4,
//     // compareFalse = 5,
//     // compareTrue = 6,
//     // authMethodNotSupported = 7,
//     // strongerAuthRequired = 8,
//     // referral = 10,
//     // adminLimitExceeded = 11,
//     // unavailableCriticalExtension = 12,
//     // confidentialityRequired = 13,
//     // saslBindInProgress = 14,
//     // noSuchAttribute = 16,
//     // undefinedAttributeType = 17,
//     // inappropriateMatching = 18,
//     // constraintViolation = 19,
//     // attributeOrValueExists = 20,
//     // invalidAttributeSyntax = 21,
//     // noSuchObject = 32,
//     // aliasProblem = 33,
//     // invalidDNSyntax = 34,
//     // aliasDereferencingProblem = 36,
//     // inappropriateAuthentication = 48,
//     // invalidCredentials = 49,
//     // insufficientAccessRights = 50,
//     // busy = 51,
//     // unavailable = 52,
//     // unwillingToPerform = 53,
//     // loopDetect = 54,
//     // namingViolation = 64,
//     // objectClassViolation = 65,
//     // notAllowedOnNonLeaf = 66,
//     // notAllowedOnRDN = 67,
//     // entryAlreadyExists = 68,
//     // objectClassModsProhibited = 69,
//     // affectsMultipleDSAs = 71,
//     // other = 80,
//         }
//     }
// }
