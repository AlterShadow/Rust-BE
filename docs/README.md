
# auth Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10020|Login|address, signature_text, signature, service_code, device_id, device_os|address, user_public_id, user_token, admin_token||
|10010|Signup|address, password, email, phone, agreed_tos, agreed_privacy|address, user_public_id||
|10030|Authorize|address, token, service_code, device_id, device_os|success||

# user Server
ID: 2
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|20040|TransferOrganizationOwner|organization_id, transfer_organization_owner_key, new_owner_user_id|||
|20042|ListOrganizationMembership||memberships||

# admin Server
ID: 3
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
