
# auth Server
ID: 1
## Endpoints
|Method Code|Method Name|Parameters|Response|Description|
|-----------|-----------|----------|--------|-----------|
|10020|Login|username, password, service_code, device_id, device_os|username, user_public_id, user_token, admin_token||
|10010|Signup|username, password, email, phone, agreed_tos, agreed_privacy|username, user_public_id||
|10030|Authorize|username, token, service_code, device_id, device_os|success||

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
