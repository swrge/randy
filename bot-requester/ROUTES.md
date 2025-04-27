# Routing

### HTTP Methods
  - **GET** for retrieval (e.g., GetUser, ListVoiceRegions).
  - **POST** for creation (e.g., CreateWebhook, ExecuteWebhook).
  - **PATCH** for updates (e.g., ModifyCurrentUser, EditWebhookMessage).
  - **PUT** for idempotent updates or additions (e.g., AddGuildMemberRole, UpdateCurrentUserApplicationRoleConnection).
  - **DELETE** for removal (e.g., DeleteWebhook, RemoveGuildMember).

### RouteID
- Matches the endpoint name directly (e.g., GetGuildMember, CreateMessage) for clarity and consistency with Disgo's rate-limiting approach. These can be adjusted if Disgo requires specific naming conventions (e.g., message_create instead of CreateMessage).

### URLBuilder
- Uses fmt.Sprintf to construct the endpoint path with variables from the vars map, ensuring all required parameters (e.g., guild_id, user_id) are included.
- Static endpoints (e.g., /gateway) return the path as-is since no variables are needed.

### Completeness
- This list includes all endpoints from your provided constants, starting from EndpointGetCurrentUser onward to EndpointGetCurrentAuthorizationInformation, completing the previous partial response.
