package main

import "fmt"

const (
	DiscordURL                                             = "https://discord.com"
	BaseURL                                                = "/api/"
	DiscordBaseURL                                         = DiscordURL + BaseURL
	EndpointGetGlobalApplicationCommands                   = "applications/{application_id}/commands"
	EndpointCreateGlobalApplicationCommand                 = "applications/{application_id}/commands"
	EndpointGetGlobalApplicationCommand                    = "applications/{application_id}/commands/{command_id}"
	EndpointEditGlobalApplicationCommand                   = "applications/{application_id}/commands/{command_id}"
	EndpointDeleteGlobalApplicationCommand                 = "applications/{application_id}/commands/{command_id}"
	EndpointBulkOverwriteGlobalApplicationCommands         = "applications/{application_id}/commands"
	EndpointGetGuildApplicationCommands                    = "applications/{application_id}/guilds/{guild_id}/commands"
	EndpointCreateGuildApplicationCommand                  = "applications/{application_id}/guilds/{guild_id}/commands"
	EndpointGetGuildApplicationCommand                     = "applications/{application_id}/guilds/{guild_id}/commands/{command_id}"
	EndpointEditGuildApplicationCommand                    = "applications/{application_id}/guilds/{guild_id}/commands/{command_id}"
	EndpointDeleteGuildApplicationCommand                  = "applications/{application_id}/guilds/{guild_id}/commands/{command_id}"
	EndpointBulkOverwriteGuildApplicationCommands          = "applications/{application_id}/guilds/{guild_id}/commands"
	EndpointGetGuildApplicationCommandPermissions          = "applications/{application_id}/guilds/{guild_id}/commands/permissions"
	EndpointGetApplicationCommandPermissions               = "applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions"
	EndpointEditApplicationCommandPermissions              = "applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions"
	EndpointBatchEditApplicationCommandPermissions         = "applications/{application_id}/guilds/{guild_id}/commands/permissions"
	EndpointCreateInteractionResponse                      = "interactions/{interaction_id}/{interaction_token}/callback"
	EndpointGetOriginalInteractionResponse                 = "webhooks/{application_id}/{interaction_token}/messages/@original"
	EndpointEditOriginalInteractionResponse                = "webhooks/{application_id}/{interaction_token}/messages/@original"
	EndpointDeleteOriginalInteractionResponse              = "webhooks/{application_id}/{interaction_token}/messages/@original"
	EndpointCreateFollowupMessage                          = "webhooks/{application_id}/{interaction_token}"
	EndpointGetFollowupMessage                             = "webhooks/{application_id}/{interaction_token}/messages/{message_id}"
	EndpointEditFollowupMessage                            = "webhooks/{application_id}/{interaction_token}/messages/{message_id}"
	EndpointDeleteFollowupMessage                          = "webhooks/{application_id}/{interaction_token}/messages/{message_id}"
	EndpointGetCurrentApplication                          = "applications/@me"
	EndpointEditCurrentApplication                         = "applications/@me"
	EndpointGetApplicationActivityInstance                 = "applications/{application_id}/activity-instances/{instance_id}"
	EndpointGetApplicationRoleConnectionMetadataRecords    = "applications/{application_id}/role-connections/metadata"
	EndpointUpdateApplicationRoleConnectionMetadataRecords = "applications/{application_id}/role-connections/metadata"
	EndpointGetGuildAuditLog                               = "guilds/{guild_id}/audit-logs"
	EndpointListAutoModerationRulesForGuild                = "guilds/{guild_id}/auto-moderation/rules"
	EndpointGetAutoModerationRule                          = "guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"
	EndpointCreateAutoModerationRule                       = "guilds/{guild_id}/auto-moderation/rules"
	EndpointModifyAutoModerationRule                       = "guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"
	EndpointDeleteAutoModerationRule                       = "guilds/{guild_id}/auto-moderation/rules/{auto_moderation_rule_id}"
	EndpointGetChannel                                     = "channels/{channel_id}"
	EndpointModifyChannel                                  = "channels/{channel_id}"
	EndpointDeleteCloseChannel                             = "channels/{channel_id}"
	EndpointEditChannelPermissions                         = "channels/{channel_id}/permissions/{overwrite_id}"
	EndpointGetChannelInvites                              = "channels/{channel_id}/invites"
	EndpointCreateChannelInvite                            = "channels/{channel_id}/invites"
	EndpointDeleteChannelPermission                        = "channels/{channel_id}/permissions/{overwrite_id}"
	EndpointFollowAnnouncementChannel                      = "channels/{channel_id}/followers"
	EndpointTriggerTypingIndicator                         = "channels/{channel_id}/typing"
	EndpointGetPinnedMessages                              = "channels/{channel_id}/pins"
	EndpointPinMessage                                     = "channels/{channel_id}/pins/{message_id}"
	EndpointUnpinMessage                                   = "channels/{channel_id}/pins/{message_id}"
	EndpointGroupDMAddRecipient                            = "channels/{channel_id}/recipients/{user_id}"
	EndpointGroupDMRemoveRecipient                         = "channels/{channel_id}/recipients/{user_id}"
	EndpointStartThreadfromMessage                         = "channels/{channel_id}/messages/{message_id}/threads"
	EndpointStartThreadwithoutMessage                      = "channels/{channel_id}/threads"
	EndpointStartThreadinForumChannel                      = "channels/{channel_id}/threads"
	EndpointJoinThread                                     = "channels/{channel_id}/thread-members/@me"
	EndpointAddThreadMember                                = "channels/{channel_id}/thread-members/{user_id}"
	EndpointLeaveThread                                    = "channels/{channel_id}/thread-members/@me"
	EndpointRemoveThreadMember                             = "channels/{channel_id}/thread-members/{user_id}"
	EndpointGetThreadMember                                = "channels/{channel_id}/thread-members/{user_id}"
	EndpointListThreadMembers                              = "channels/{channel_id}/thread-members"
	EndpointListPublicArchivedThreads                      = "channels/{channel_id}/threads/archived/public"
	EndpointListPrivateArchivedThreads                     = "channels/{channel_id}/threads/archived/private"
	EndpointListJoinedPrivateArchivedThreads               = "channels/{channel_id}/users/@me/threads/archived/private"
	EndpointListGuildEmojis                                = "guilds/{guild_id}/emojis"
	EndpointGetGuildEmoji                                  = "guilds/{guild_id}/emojis/{emoji_id}"
	EndpointCreateGuildEmoji                               = "guilds/{guild_id}/emojis"
	EndpointModifyGuildEmoji                               = "guilds/{guild_id}/emojis/{emoji_id}"
	EndpointDeleteGuildEmoji                               = "guilds/{guild_id}/emojis/{emoji_id}"
	EndpointListApplicationEmojis                          = "applications/{application_id}/emojis"
	EndpointGetApplicationEmoji                            = "applications/{application_id}/emojis/{emoji_id}"
	EndpointCreateApplicationEmoji                         = "applications/{application_id}/emojis"
	EndpointModifyApplicationEmoji                         = "applications/{application_id}/emojis/{emoji_id}"
	EndpointDeleteApplicationEmoji                         = "applications/{application_id}/emojis/{emoji_id}"
	EndpointListEntitlements                               = "applications/{application_id}/entitlements"
	EndpointGetEntitlement                                 = "applications/{application_id}/entitlements/{entitlement_id}"
	EndpointConsumeEntitlement                             = "applications/{application_id}/entitlements/{entitlement_id}/consume"
	EndpointCreateTestEntitlement                          = "applications/{application_id}/entitlements"
	EndpointDeleteTestEntitlement                          = "applications/{application_id}/entitlements/{entitlement_id}"
	EndpointCreateGuild                                    = "guilds"
	EndpointGetGuild                                       = "guilds/{guild_id}"
	EndpointGetGuildPreview                                = "guilds/{guild_id}/preview"
	EndpointModifyGuild                                    = "guilds/{guild_id}"
	EndpointDeleteGuild                                    = "guilds/{guild_id}"
	EndpointGetGuildChannels                               = "guilds/{guild_id}/channels"
	EndpointCreateGuildChannel                             = "guilds/{guild_id}/channels"
	EndpointModifyGuildChannelPositions                    = "guilds/{guild_id}/channels"
	EndpointListActiveGuildThreads                         = "guilds/{guild_id}/threads/active"
	EndpointGetGuildMember                                 = "guilds/{guild_id}/members/{user_id}"
	EndpointListGuildMembers                               = "guilds/{guild_id}/members"
	EndpointSearchGuildMembers                             = "guilds/{guild_id}/members/search"
	EndpointAddGuildMember                                 = "guilds/{guild_id}/members/{user_id}"
	EndpointModifyGuildMember                              = "guilds/{guild_id}/members/{user_id}"
	EndpointModifyCurrentMember                            = "guilds/{guild_id}/members/@me"
	EndpointModifyCurrentUserNick                          = "guilds/{guild_id}/members/@me/nick"
	EndpointAddGuildMemberRole                             = "guilds/{guild_id}/members/{user_id}/roles/{role_id}"
	EndpointRemoveGuildMemberRole                          = "guilds/{guild_id}/members/{user_id}/roles/{role_id}"
	EndpointRemoveGuildMember                              = "guilds/{guild_id}/members/{user_id}"
	EndpointGetGuildBans                                   = "guilds/{guild_id}/bans"
	EndpointGetGuildBan                                    = "guilds/{guild_id}/bans/{user_id}"
	EndpointCreateGuildBan                                 = "guilds/{guild_id}/bans/{user_id}"
	EndpointRemoveGuildBan                                 = "guilds/{guild_id}/bans/{user_id}"
	EndpointGetGuildRoles                                  = "guilds/{guild_id}/roles"
	EndpointGetGuildRole                                   = "guilds/{guild_id}/roles/{role_id}"
	EndpointCreateGuildRole                                = "guilds/{guild_id}/roles"
	EndpointModifyGuildRolePositions                       = "guilds/{guild_id}/roles"
	EndpointModifyGuildRole                                = "guilds/{guild_id}/roles/{role_id}"
	EndpointModifyGuildMFALevel                            = "guilds/{guild_id}/mfa"
	EndpointDeleteGuildRole                                = "guilds/{guild_id}/roles/{role_id}"
	EndpointGetGuildPruneCount                             = "guilds/{guild_id}/prune"
	EndpointBeginGuildPrune                                = "guilds/{guild_id}/prune"
	EndpointGetGuildVoiceRegions                           = "guilds/{guild_id}/regions"
	EndpointGetGuildInvites                                = "guilds/{guild_id}/invites"
	EndpointGetGuildIntegrations                           = "guilds/{guild_id}/integrations"
	EndpointDeleteGuildIntegration                         = "guilds/{guild_id}/integrations/{integration_id}"
	EndpointGetGuildWidgetSettings                         = "guilds/{guild_id}/widget"
	EndpointModifyGuildWidget                              = "guilds/{guild_id}/widget"
	EndpointGetGuildWidget                                 = "guilds/{guild_id}/widget.json"
	EndpointGetGuildVanityURL                              = "guilds/{guild_id}/vanity-url"
	EndpointGetGuildWidgetImage                            = "guilds/{guild_id}/widget.png"
	EndpointGetGuildWelcomeScreen                          = "guilds/{guild_id}/welcome-screen"
	EndpointModifyGuildWelcomeScreen                       = "guilds/{guild_id}/welcome-screen"
	EndpointGetGuildOnboarding                             = "guilds/{guild_id}/onboarding"
	EndpointModifyGuildOnboarding                          = "guilds/{guild_id}/onboarding"
	EndpointListScheduledEventsforGuild                    = "guilds/{guild_id}/scheduled-events"
	EndpointCreateGuildScheduledEvent                      = "guilds/{guild_id}/scheduled-events"
	EndpointGetGuildScheduledEvent                         = "guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}"
	EndpointModifyGuildScheduledEvent                      = "guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}"
	EndpointDeleteGuildScheduledEvent                      = "guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}"
	EndpointGetGuildScheduledEventUsers                    = "guilds/{guild_id}/scheduled-events/{guild_scheduled_event_id}/users"
	EndpointGetGuildTemplate                               = "guilds/templates/{template_code}"
	EndpointCreateGuildfromGuildTemplate                   = "guilds/templates/{template_code}"
	EndpointGetGuildTemplates                              = "guilds/{guild_id}/templates"
	EndpointCreateGuildTemplate                            = "guilds/{guild_id}/templates"
	EndpointSyncGuildTemplate                              = "guilds/{guild_id}/templates/{template_code}"
	EndpointModifyGuildTemplate                            = "guilds/{guild_id}/templates/{template_code}"
	EndpointDeleteGuildTemplate                            = "guilds/{guild_id}/templates/{template_code}"
	EndpointGetInvite                                      = "invites/{invite_code}"
	EndpointDeleteInvite                                   = "invites/{invite_code}"
	EndpointGetChannelMessages                             = "channels/{channel_id}/messages"
	EndpointGetChannelMessage                              = "channels/{channel_id}/messages/{message_id}"
	EndpointCreateMessage                                  = "channels/{channel_id}/messages"
	EndpointCrosspostMessage                               = "channels/{channel_id}/messages/{message_id}/crosspost"
	EndpointCreateReaction                                 = "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me"
	EndpointDeleteOwnReaction                              = "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me"
	EndpointDeleteUserReaction                             = "channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}"
	EndpointGetReactions                                   = "channels/{channel_id}/messages/{message_id}/reactions/{emoji}"
	EndpointDeleteAllReactions                             = "channels/{channel_id}/messages/{message_id}/reactions"
	EndpointDeleteAllReactionsforEmoji                     = "channels/{channel_id}/messages/{message_id}/reactions/{emoji}"
	EndpointEditMessage                                    = "channels/{channel_id}/messages/{message_id}"
	EndpointDeleteMessage                                  = "channels/{channel_id}/messages/{message_id}"
	EndpointBulkDeleteMessages                             = "channels/{channel_id}/messages/bulk-delete"
	EndpointGetAnswerVoters                                = "channels/{channel_id}/polls/{message_id}/answers/{answer_id}"
	EndpointEndPoll                                        = "channels/{channel_id}/polls/{message_id}/expire"
	EndpointListSKUs                                       = "applications/{application_id}/skus"
	EndpointSendSoundboardSound                            = "channels/{channel_id}/send-soundboard-sound"
	EndpointListDefaultSoundboardSounds                    = "soundboard-default-sounds"
	EndpointListGuildSoundboardSounds                      = "guilds/{guild_id}/soundboard-sounds"
	EndpointGetGuildSoundboardSound                        = "guilds/{guild_id}/soundboard-sounds/{sound_id}"
	EndpointCreateGuildSoundboardSound                     = "guilds/{guild_id}/soundboard-sounds"
	EndpointModifyGuildSoundboardSound                     = "guilds/{guild_id}/soundboard-sounds/{sound_id}"
	EndpointDeleteGuildSoundboardSound                     = "guilds/{guild_id}/soundboard-sounds/{sound_id}"
	EndpointCreateStageInstance                            = "stage-instances"
	EndpointGetStageInstance                               = "stage-instances/{channel_id}"
	EndpointModifyStageInstance                            = "stage-instances/{channel_id}"
	EndpointDeleteStageInstance                            = "stage-instances/{channel_id}"
	EndpointGetSticker                                     = "stickers/{sticker_id}"
	EndpointListStickerPacks                               = "sticker-packs"
	EndpointGetStickerPack                                 = "sticker-packs/{pack_id}"
	EndpointListGuildStickers                              = "guilds/{guild_id}/stickers"
	EndpointGetGuildSticker                                = "guilds/{guild_id}/stickers/{sticker_id}"
	EndpointCreateGuildSticker                             = "guilds/{guild_id}/stickers"
	EndpointModifyGuildSticker                             = "guilds/{guild_id}/stickers/{sticker_id}"
	EndpointDeleteGuildSticker                             = "guilds/{guild_id}/stickers/{sticker_id}"
	EndpointListSKUSubscriptions                           = "skus/{sku_id}/subscriptions"
	EndpointGetSKUSubscription                             = "skus/{sku_id}/subscriptions/{subscription_id}"
	EndpointModifyCurrentUserVoiceState                    = "guilds/{guild_id}/voice-states/@me"
	EndpointModifyUserVoiceState                           = "guilds/{guild_id}/voice-states/{user_id}"
	EndpointGetCurrentUser                                 = "users/@me"
	EndpointGetUser                                        = "users/{user_id}"
	EndpointModifyCurrentUser                              = "users/@me"
	EndpointGetCurrentUserGuilds                           = "users/@me/guilds"
	EndpointGetCurrentUserGuildMember                      = "users/@me/guilds/{guild_id}/member"
	EndpointLeaveGuild                                     = "users/@me/guilds/{guild_id}"
	EndpointCreateDM                                       = "users/@me/channels"
	EndpointCreateGroupDM                                  = "users/@me/channels"
	EndpointGetCurrentUserConnections                      = "users/@me/connections"
	EndpointGetCurrentUserApplicationRoleConnection        = "users/@me/applications/{application_id}/role-connection"
	EndpointUpdateCurrentUserApplicationRoleConnection     = "users/@me/applications/{application_id}/role-connection"
	EndpointListVoiceRegions                               = "voice/regions"
	EndpointCreateWebhook                                  = "channels/{channel_id}/webhooks"
	EndpointGetChannelWebhooks                             = "channels/{channel_id}/webhooks"
	EndpointGetGuildWebhooks                               = "guilds/{guild_id}/webhooks"
	EndpointGetWebhook                                     = "webhooks/{webhook_id}"
	EndpointGetWebhookwithToken                            = "webhooks/{webhook_id}/{webhook_token}"
	EndpointModifyWebhook                                  = "webhooks/{webhook_id}"
	EndpointModifyWebhookwithToken                         = "webhooks/{webhook_id}/{webhook_token}"
	EndpointDeleteWebhook                                  = "webhooks/{webhook_id}"
	EndpointDeleteWebhookwithToken                         = "webhooks/{webhook_id}/{webhook_token}"
	EndpointExecuteWebhook                                 = "webhooks/{webhook_id}/{webhook_token}"
	EndpointExecuteSlackCompatibleWebhook                  = "webhooks/{webhook_id}/{webhook_token}/slack"
	EndpointExecuteGitHubCompatibleWebhook                 = "webhooks/{webhook_id}/{webhook_token}/github"
	EndpointGetWebhookMessage                              = "webhooks/{webhook_id}/{webhook_token}/messages/{message_id}"
	EndpointEditWebhookMessage                             = "webhooks/{webhook_id}/{webhook_token}/messages/{message_id}"
	EndpointDeleteWebhookMessage                           = "webhooks/{webhook_id}/{webhook_token}/messages/{message_id}"
	EndpointGetGateway                                     = "gateway"
	EndpointGetGatewayBot                                  = "gateway/bot"
	EndpointAuthorizationURL                               = "oauth2/authorize"
	EndpointTokenURL                                       = "oauth2/token"
	EndpointTokenRevocationURL                             = "oauth2/token/revoke"
	EndpointGetCurrentBotApplicationInformation            = "oauth2/applications/@me"
	EndpointGetCurrentAuthorizationInformation             = "oauth2/@me"
)

type RouteConfig struct {
	Method      string
	PathPattern string
	RouteID     string
	URLBuilder  func(vars map[string]string) string
}

// Define route configurations for Discord API endpoints
var routeConfigs = []RouteConfig{
	// Application Commands (Global)
	{
		Method:      "GET",
		PathPattern: EndpointGetGlobalApplicationCommands,
		RouteID:     "GetGlobalApplicationCommands",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands", vars["application_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGlobalApplicationCommand,
		RouteID:     "CreateGlobalApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands", vars["application_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGlobalApplicationCommand,
		RouteID:     "GetGlobalApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands/%s", vars["application_id"], vars["command_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditGlobalApplicationCommand,
		RouteID:     "EditGlobalApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands/%s", vars["application_id"], vars["command_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGlobalApplicationCommand,
		RouteID:     "DeleteGlobalApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands/%s", vars["application_id"], vars["command_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointBulkOverwriteGlobalApplicationCommands,
		RouteID:     "BulkOverwriteGlobalApplicationCommands",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/commands", vars["application_id"])
		},
	},

	// Application Commands (Guild)
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildApplicationCommands,
		RouteID:     "GetGuildApplicationCommands",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands", vars["application_id"], vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildApplicationCommand,
		RouteID:     "CreateGuildApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands", vars["application_id"], vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildApplicationCommand,
		RouteID:     "GetGuildApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/%s", vars["application_id"], vars["guild_id"], vars["command_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditGuildApplicationCommand,
		RouteID:     "EditGuildApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/%s", vars["application_id"], vars["guild_id"], vars["command_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildApplicationCommand,
		RouteID:     "DeleteGuildApplicationCommand",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/%s", vars["application_id"], vars["guild_id"], vars["command_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointBulkOverwriteGuildApplicationCommands,
		RouteID:     "BulkOverwriteGuildApplicationCommands",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands", vars["application_id"], vars["guild_id"])
		},
	},

	// Application Command Permissions
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildApplicationCommandPermissions,
		RouteID:     "GetGuildApplicationCommandPermissions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/permissions", vars["application_id"], vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetApplicationCommandPermissions,
		RouteID:     "GetApplicationCommandPermissions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/%s/permissions", vars["application_id"], vars["guild_id"], vars["command_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointEditApplicationCommandPermissions,
		RouteID:     "EditApplicationCommandPermissions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/%s/permissions", vars["application_id"], vars["guild_id"], vars["command_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointBatchEditApplicationCommandPermissions,
		RouteID:     "BatchEditApplicationCommandPermissions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/guilds/%s/commands/permissions", vars["application_id"], vars["guild_id"])
		},
	},

	// Interaction Responses
	{
		Method:      "POST",
		PathPattern: EndpointCreateInteractionResponse,
		RouteID:     "CreateInteractionResponse",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("interactions/%s/%s/callback", vars["interaction_id"], vars["interaction_token"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetOriginalInteractionResponse,
		RouteID:     "GetOriginalInteractionResponse",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/@original", vars["application_id"], vars["interaction_token"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditOriginalInteractionResponse,
		RouteID:     "EditOriginalInteractionResponse",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/@original", vars["application_id"], vars["interaction_token"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteOriginalInteractionResponse,
		RouteID:     "DeleteOriginalInteractionResponse",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/@original", vars["application_id"], vars["interaction_token"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateFollowupMessage,
		RouteID:     "CreateFollowupMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s", vars["application_id"], vars["interaction_token"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetFollowupMessage,
		RouteID:     "GetFollowupMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["application_id"], vars["interaction_token"], vars["message_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditFollowupMessage,
		RouteID:     "EditFollowupMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["application_id"], vars["interaction_token"], vars["message_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteFollowupMessage,
		RouteID:     "DeleteFollowupMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["application_id"], vars["interaction_token"], vars["message_id"])
		},
	},

	// Application
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentApplication,
		RouteID:     "GetCurrentApplication",
		URLBuilder: func(vars map[string]string) string {
			return "applications/@me"
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditCurrentApplication,
		RouteID:     "EditCurrentApplication",
		URLBuilder: func(vars map[string]string) string {
			return "applications/@me"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetApplicationActivityInstance,
		RouteID:     "GetApplicationActivityInstance",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/activity-instances/%s", vars["application_id"], vars["instance_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetApplicationRoleConnectionMetadataRecords,
		RouteID:     "GetApplicationRoleConnectionMetadataRecords",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/role-connections/metadata", vars["application_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointUpdateApplicationRoleConnectionMetadataRecords,
		RouteID:     "UpdateApplicationRoleConnectionMetadataRecords",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/role-connections/metadata", vars["application_id"])
		},
	},

	// Audit Log
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildAuditLog,
		RouteID:     "GetGuildAuditLog",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/audit-logs", vars["guild_id"])
		},
	},

	// Auto Moderation
	{
		Method:      "GET",
		PathPattern: EndpointListAutoModerationRulesForGuild,
		RouteID:     "ListAutoModerationRulesForGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/auto-moderation/rules", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetAutoModerationRule,
		RouteID:     "GetAutoModerationRule",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/auto-moderation/rules/%s", vars["guild_id"], vars["auto_moderation_rule_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateAutoModerationRule,
		RouteID:     "CreateAutoModerationRule",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/auto-moderation/rules", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyAutoModerationRule,
		RouteID:     "ModifyAutoModerationRule",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/auto-moderation/rules/%s", vars["guild_id"], vars["auto_moderation_rule_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteAutoModerationRule,
		RouteID:     "DeleteAutoModerationRule",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/auto-moderation/rules/%s", vars["guild_id"], vars["auto_moderation_rule_id"])
		},
	},

	// Channel
	{
		Method:      "GET",
		PathPattern: EndpointGetChannel,
		RouteID:     "GetChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s", vars["channel_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyChannel,
		RouteID:     "ModifyChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s", vars["channel_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteCloseChannel,
		RouteID:     "DeleteCloseChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s", vars["channel_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointEditChannelPermissions,
		RouteID:     "EditChannelPermissions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/permissions/%s", vars["channel_id"], vars["overwrite_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetChannelInvites,
		RouteID:     "GetChannelInvites",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/invites", vars["channel_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateChannelInvite,
		RouteID:     "CreateChannelInvite",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/invites", vars["channel_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteChannelPermission,
		RouteID:     "DeleteChannelPermission",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/permissions/%s", vars["channel_id"], vars["overwrite_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointFollowAnnouncementChannel,
		RouteID:     "FollowAnnouncementChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/followers", vars["channel_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointTriggerTypingIndicator,
		RouteID:     "TriggerTypingIndicator",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/typing", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetPinnedMessages,
		RouteID:     "GetPinnedMessages",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/pins", vars["channel_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointPinMessage,
		RouteID:     "PinMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/pins/%s", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointUnpinMessage,
		RouteID:     "UnpinMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/pins/%s", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointGroupDMAddRecipient,
		RouteID:     "GroupDMAddRecipient",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/recipients/%s", vars["channel_id"], vars["user_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointGroupDMRemoveRecipient,
		RouteID:     "GroupDMRemoveRecipient",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/recipients/%s", vars["channel_id"], vars["user_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointStartThreadfromMessage,
		RouteID:     "StartThreadfromMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/threads", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointStartThreadwithoutMessage,
		RouteID:     "StartThreadwithoutMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/threads", vars["channel_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointStartThreadinForumChannel,
		RouteID:     "StartThreadinForumChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/threads", vars["channel_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointJoinThread,
		RouteID:     "JoinThread",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members/@me", vars["channel_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointAddThreadMember,
		RouteID:     "AddThreadMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members/%s", vars["channel_id"], vars["user_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointLeaveThread,
		RouteID:     "LeaveThread",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members/@me", vars["channel_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointRemoveThreadMember,
		RouteID:     "RemoveThreadMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members/%s", vars["channel_id"], vars["user_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetThreadMember,
		RouteID:     "GetThreadMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members/%s", vars["channel_id"], vars["user_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListThreadMembers,
		RouteID:     "ListThreadMembers",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/thread-members", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListPublicArchivedThreads,
		RouteID:     "ListPublicArchivedThreads",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/threads/archived/public", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListPrivateArchivedThreads,
		RouteID:     "ListPrivateArchivedThreads",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/threads/archived/private", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListJoinedPrivateArchivedThreads,
		RouteID:     "ListJoinedPrivateArchivedThreads",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/users/@me/threads/archived/private", vars["channel_id"])
		},
	},

	// Guild Emojis
	{
		Method:      "GET",
		PathPattern: EndpointListGuildEmojis,
		RouteID:     "ListGuildEmojis",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/emojis", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildEmoji,
		RouteID:     "GetGuildEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/emojis/%s", vars["guild_id"], vars["emoji_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildEmoji,
		RouteID:     "CreateGuildEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/emojis", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildEmoji,
		RouteID:     "ModifyGuildEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/emojis/%s", vars["guild_id"], vars["emoji_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildEmoji,
		RouteID:     "DeleteGuildEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/emojis/%s", vars["guild_id"], vars["emoji_id"])
		},
	},

	// Application Emojis
	{
		Method:      "GET",
		PathPattern: EndpointListApplicationEmojis,
		RouteID:     "ListApplicationEmojis",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/emojis", vars["application_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetApplicationEmoji,
		RouteID:     "GetApplicationEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/emojis/%s", vars["application_id"], vars["emoji_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateApplicationEmoji,
		RouteID:     "CreateApplicationEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/emojis", vars["application_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyApplicationEmoji,
		RouteID:     "ModifyApplicationEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/emojis/%s", vars["application_id"], vars["emoji_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteApplicationEmoji,
		RouteID:     "DeleteApplicationEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/emojis/%s", vars["application_id"], vars["emoji_id"])
		},
	},

	// Entitlements
	{
		Method:      "GET",
		PathPattern: EndpointListEntitlements,
		RouteID:     "ListEntitlements",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/entitlements", vars["application_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetEntitlement,
		RouteID:     "GetEntitlement",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/entitlements/%s", vars["application_id"], vars["entitlement_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointConsumeEntitlement,
		RouteID:     "ConsumeEntitlement",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/entitlements/%s/consume", vars["application_id"], vars["entitlement_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateTestEntitlement,
		RouteID:     "CreateTestEntitlement",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/entitlements", vars["application_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteTestEntitlement,
		RouteID:     "DeleteTestEntitlement",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/entitlements/%s", vars["application_id"], vars["entitlement_id"])
		},
	},

	// Guild
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuild,
		RouteID:     "CreateGuild",
		URLBuilder: func(vars map[string]string) string {
			return "guilds"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuild,
		RouteID:     "GetGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildPreview,
		RouteID:     "GetGuildPreview",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/preview", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuild,
		RouteID:     "ModifyGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s", vars["guild_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuild,
		RouteID:     "DeleteGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildChannels,
		RouteID:     "GetGuildChannels",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/channels", vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildChannel,
		RouteID:     "CreateGuildChannel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/channels", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildChannelPositions,
		RouteID:     "ModifyGuildChannelPositions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/channels", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListActiveGuildThreads,
		RouteID:     "ListActiveGuildThreads",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/threads/active", vars["guild_id"])
		},
	},

	// Guild Members
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildMember,
		RouteID:     "GetGuildMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s", vars["guild_id"], vars["user_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListGuildMembers,
		RouteID:     "ListGuildMembers",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointSearchGuildMembers,
		RouteID:     "SearchGuildMembers",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/search", vars["guild_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointAddGuildMember,
		RouteID:     "AddGuildMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s", vars["guild_id"], vars["user_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildMember,
		RouteID:     "ModifyGuildMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s", vars["guild_id"], vars["user_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyCurrentMember,
		RouteID:     "ModifyCurrentMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/@me", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyCurrentUserNick,
		RouteID:     "ModifyCurrentUserNick",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/@me/nick", vars["guild_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointAddGuildMemberRole,
		RouteID:     "AddGuildMemberRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s/roles/%s", vars["guild_id"], vars["user_id"], vars["role_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointRemoveGuildMemberRole,
		RouteID:     "RemoveGuildMemberRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s/roles/%s", vars["guild_id"], vars["user_id"], vars["role_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointRemoveGuildMember,
		RouteID:     "RemoveGuildMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/members/%s", vars["guild_id"], vars["user_id"])
		},
	},

	// Guild Bans
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildBans,
		RouteID:     "GetGuildBans",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/bans", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildBan,
		RouteID:     "GetGuildBan",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/bans/%s", vars["guild_id"], vars["user_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointCreateGuildBan,
		RouteID:     "CreateGuildBan",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/bans/%s", vars["guild_id"], vars["user_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointRemoveGuildBan,
		RouteID:     "RemoveGuildBan",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/bans/%s", vars["guild_id"], vars["user_id"])
		},
	},

	// Guild Roles
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildRoles,
		RouteID:     "GetGuildRoles",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildRole,
		RouteID:     "GetGuildRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles/%s", vars["guild_id"], vars["role_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildRole,
		RouteID:     "CreateGuildRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildRolePositions,
		RouteID:     "ModifyGuildRolePositions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildRole,
		RouteID:     "ModifyGuildRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles/%s", vars["guild_id"], vars["role_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointModifyGuildMFALevel,
		RouteID:     "ModifyGuildMFALevel",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/mfa", vars["guild_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildRole,
		RouteID:     "DeleteGuildRole",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/roles/%s", vars["guild_id"], vars["role_id"])
		},
	},

	// Guild Prune
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildPruneCount,
		RouteID:     "GetGuildPruneCount",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/prune", vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointBeginGuildPrune,
		RouteID:     "BeginGuildPrune",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/prune", vars["guild_id"])
		},
	},

	// Guild Miscellaneous
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildVoiceRegions,
		RouteID:     "GetGuildVoiceRegions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/regions", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildInvites,
		RouteID:     "GetGuildInvites",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/invites", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildIntegrations,
		RouteID:     "GetGuildIntegrations",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/integrations", vars["guild_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildIntegration,
		RouteID:     "DeleteGuildIntegration",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/integrations/%s", vars["guild_id"], vars["integration_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildWidgetSettings,
		RouteID:     "GetGuildWidgetSettings",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/widget", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildWidget,
		RouteID:     "ModifyGuildWidget",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/widget", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildWidget,
		RouteID:     "GetGuildWidget",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/widget.json", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildVanityURL,
		RouteID:     "GetGuildVanityURL",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/vanity-url", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildWidgetImage,
		RouteID:     "GetGuildWidgetImage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/widget.png", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildWelcomeScreen,
		RouteID:     "GetGuildWelcomeScreen",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/welcome-screen", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildWelcomeScreen,
		RouteID:     "ModifyGuildWelcomeScreen",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/welcome-screen", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildOnboarding,
		RouteID:     "GetGuildOnboarding",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/onboarding", vars["guild_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointModifyGuildOnboarding,
		RouteID:     "ModifyGuildOnboarding",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/onboarding", vars["guild_id"])
		},
	},

	// Guild Scheduled Events
	{
		Method:      "GET",
		PathPattern: EndpointListScheduledEventsforGuild,
		RouteID:     "ListScheduledEventsforGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events", vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildScheduledEvent,
		RouteID:     "CreateGuildScheduledEvent",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildScheduledEvent,
		RouteID:     "GetGuildScheduledEvent",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events/%s", vars["guild_id"], vars["guild_scheduled_event_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildScheduledEvent,
		RouteID:     "ModifyGuildScheduledEvent",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events/%s", vars["guild_id"], vars["guild_scheduled_event_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildScheduledEvent,
		RouteID:     "DeleteGuildScheduledEvent",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events/%s", vars["guild_id"], vars["guild_scheduled_event_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildScheduledEventUsers,
		RouteID:     "GetGuildScheduledEventUsers",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/scheduled-events/%s/users", vars["guild_id"], vars["guild_scheduled_event_id"])
		},
	},

	// Guild Templates
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildTemplate,
		RouteID:     "GetGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/templates/%s", vars["template_code"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildfromGuildTemplate,
		RouteID:     "CreateGuildfromGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/templates/%s", vars["template_code"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildTemplates,
		RouteID:     "GetGuildTemplates",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/templates", vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildTemplate,
		RouteID:     "CreateGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/templates", vars["guild_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointSyncGuildTemplate,
		RouteID:     "SyncGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/templates/%s", vars["guild_id"], vars["template_code"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildTemplate,
		RouteID:     "ModifyGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/templates/%s", vars["guild_id"], vars["template_code"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildTemplate,
		RouteID:     "DeleteGuildTemplate",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/templates/%s", vars["guild_id"], vars["template_code"])
		},
	},

	// Invite
	{
		Method:      "GET",
		PathPattern: EndpointGetInvite,
		RouteID:     "GetInvite",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("invites/%s", vars["invite_code"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteInvite,
		RouteID:     "DeleteInvite",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("invites/%s", vars["invite_code"])
		},
	},

	// Message
	{
		Method:      "GET",
		PathPattern: EndpointGetChannelMessages,
		RouteID:     "GetChannelMessages",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetChannelMessage,
		RouteID:     "GetChannelMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateMessage,
		RouteID:     "CreateMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages", vars["channel_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCrosspostMessage,
		RouteID:     "CrosspostMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/crosspost", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointCreateReaction,
		RouteID:     "CreateReaction",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions/%s/@me", vars["channel_id"], vars["message_id"], vars["emoji"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteOwnReaction,
		RouteID:     "DeleteOwnReaction",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions/%s/@me", vars["channel_id"], vars["message_id"], vars["emoji"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteUserReaction,
		RouteID:     "DeleteUserReaction",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions/%s/%s", vars["channel_id"], vars["message_id"], vars["emoji"], vars["user_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetReactions,
		RouteID:     "GetReactions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions/%s", vars["channel_id"], vars["message_id"], vars["emoji"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteAllReactions,
		RouteID:     "DeleteAllReactions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteAllReactionsforEmoji,
		RouteID:     "DeleteAllReactionsforEmoji",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s/reactions/%s", vars["channel_id"], vars["message_id"], vars["emoji"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditMessage,
		RouteID:     "EditMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteMessage,
		RouteID:     "DeleteMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/%s", vars["channel_id"], vars["message_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointBulkDeleteMessages,
		RouteID:     "BulkDeleteMessages",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/messages/bulk-delete", vars["channel_id"])
		},
	},

	// Polls
	{
		Method:      "GET",
		PathPattern: EndpointGetAnswerVoters,
		RouteID:     "GetAnswerVoters",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/polls/%s/answers/%s", vars["channel_id"], vars["message_id"], vars["answer_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointEndPoll,
		RouteID:     "EndPoll",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/polls/%s/expire", vars["channel_id"], vars["message_id"])
		},
	},

	// SKUs
	{
		Method:      "GET",
		PathPattern: EndpointListSKUs,
		RouteID:     "ListSKUs",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("applications/%s/skus", vars["application_id"])
		},
	},

	// Soundboard
	{
		Method:      "POST",
		PathPattern: EndpointSendSoundboardSound,
		RouteID:     "SendSoundboardSound",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/send-soundboard-sound", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListDefaultSoundboardSounds,
		RouteID:     "ListDefaultSoundboardSounds",
		URLBuilder: func(vars map[string]string) string {
			return "soundboard-default-sounds"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListGuildSoundboardSounds,
		RouteID:     "ListGuildSoundboardSounds",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/soundboard-sounds", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildSoundboardSound,
		RouteID:     "GetGuildSoundboardSound",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/soundboard-sounds/%s", vars["guild_id"], vars["sound_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildSoundboardSound,
		RouteID:     "CreateGuildSoundboardSound",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/soundboard-sounds", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildSoundboardSound,
		RouteID:     "ModifyGuildSoundboardSound",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/soundboard-sounds/%s", vars["guild_id"], vars["sound_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildSoundboardSound,
		RouteID:     "DeleteGuildSoundboardSound",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/soundboard-sounds/%s", vars["guild_id"], vars["sound_id"])
		},
	},

	// Stage Instances
	{
		Method:      "POST",
		PathPattern: EndpointCreateStageInstance,
		RouteID:     "CreateStageInstance",
		URLBuilder: func(vars map[string]string) string {
			return "stage-instances"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetStageInstance,
		RouteID:     "GetStageInstance",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("stage-instances/%s", vars["channel_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyStageInstance,
		RouteID:     "ModifyStageInstance",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("stage-instances/%s", vars["channel_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteStageInstance,
		RouteID:     "DeleteStageInstance",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("stage-instances/%s", vars["channel_id"])
		},
	},

	// Stickers
	{
		Method:      "GET",
		PathPattern: EndpointGetSticker,
		RouteID:     "GetSticker",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("stickers/%s", vars["sticker_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListStickerPacks,
		RouteID:     "ListStickerPacks",
		URLBuilder: func(vars map[string]string) string {
			return "sticker-packs"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetStickerPack,
		RouteID:     "GetStickerPack",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("sticker-packs/%s", vars["pack_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointListGuildStickers,
		RouteID:     "ListGuildStickers",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/stickers", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildSticker,
		RouteID:     "GetGuildSticker",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/stickers/%s", vars["guild_id"], vars["sticker_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGuildSticker,
		RouteID:     "CreateGuildSticker",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/stickers", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyGuildSticker,
		RouteID:     "ModifyGuildSticker",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/stickers/%s", vars["guild_id"], vars["sticker_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteGuildSticker,
		RouteID:     "DeleteGuildSticker",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/stickers/%s", vars["guild_id"], vars["sticker_id"])
		},
	},

	// SKU Subscriptions
	{
		Method:      "GET",
		PathPattern: EndpointListSKUSubscriptions,
		RouteID:     "ListSKUSubscriptions",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("skus/%s/subscriptions", vars["sku_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetSKUSubscription,
		RouteID:     "GetSKUSubscription",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("skus/%s/subscriptions/%s", vars["sku_id"], vars["subscription_id"])
		},
	},

	// Voice States
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyCurrentUserVoiceState,
		RouteID:     "ModifyCurrentUserVoiceState",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/voice-states/@me", vars["guild_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyUserVoiceState,
		RouteID:     "ModifyUserVoiceState",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/voice-states/%s", vars["guild_id"], vars["user_id"])
		},
	},

	// User
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentUser,
		RouteID:     "GetCurrentUser",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetUser,
		RouteID:     "GetUser",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("users/%s", vars["user_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyCurrentUser,
		RouteID:     "ModifyCurrentUser",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentUserGuilds,
		RouteID:     "GetCurrentUserGuilds",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me/guilds"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentUserGuildMember,
		RouteID:     "GetCurrentUserGuildMember",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("users/@me/guilds/%s/member", vars["guild_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointLeaveGuild,
		RouteID:     "LeaveGuild",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("users/@me/guilds/%s", vars["guild_id"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateDM,
		RouteID:     "CreateDM",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me/channels"
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointCreateGroupDM,
		RouteID:     "CreateGroupDM",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me/channels"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentUserConnections,
		RouteID:     "GetCurrentUserConnections",
		URLBuilder: func(vars map[string]string) string {
			return "users/@me/connections"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentUserApplicationRoleConnection,
		RouteID:     "GetCurrentUserApplicationRoleConnection",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("users/@me/applications/%s/role-connection", vars["application_id"])
		},
	},
	{
		Method:      "PUT",
		PathPattern: EndpointUpdateCurrentUserApplicationRoleConnection,
		RouteID:     "UpdateCurrentUserApplicationRoleConnection",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("users/@me/applications/%s/role-connection", vars["application_id"])
		},
	},

	// Voice
	{
		Method:      "GET",
		PathPattern: EndpointListVoiceRegions,
		RouteID:     "ListVoiceRegions",
		URLBuilder: func(vars map[string]string) string {
			return "voice/regions"
		},
	},

	// Webhook
	{
		Method:      "POST",
		PathPattern: EndpointCreateWebhook,
		RouteID:     "CreateWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/webhooks", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetChannelWebhooks,
		RouteID:     "GetChannelWebhooks",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("channels/%s/webhooks", vars["channel_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGuildWebhooks,
		RouteID:     "GetGuildWebhooks",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("guilds/%s/webhooks", vars["guild_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetWebhook,
		RouteID:     "GetWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s", vars["webhook_id"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetWebhookwithToken,
		RouteID:     "GetWebhookwithToken",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyWebhook,
		RouteID:     "ModifyWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s", vars["webhook_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointModifyWebhookwithToken,
		RouteID:     "ModifyWebhookwithToken",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteWebhook,
		RouteID:     "DeleteWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s", vars["webhook_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteWebhookwithToken,
		RouteID:     "DeleteWebhookwithToken",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointExecuteWebhook,
		RouteID:     "ExecuteWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointExecuteSlackCompatibleWebhook,
		RouteID:     "ExecuteSlackCompatibleWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/slack", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointExecuteGitHubCompatibleWebhook,
		RouteID:     "ExecuteGitHubCompatibleWebhook",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/github", vars["webhook_id"], vars["webhook_token"])
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetWebhookMessage,
		RouteID:     "GetWebhookMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["webhook_id"], vars["webhook_token"], vars["message_id"])
		},
	},
	{
		Method:      "PATCH",
		PathPattern: EndpointEditWebhookMessage,
		RouteID:     "EditWebhookMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["webhook_id"], vars["webhook_token"], vars["message_id"])
		},
	},
	{
		Method:      "DELETE",
		PathPattern: EndpointDeleteWebhookMessage,
		RouteID:     "DeleteWebhookMessage",
		URLBuilder: func(vars map[string]string) string {
			return fmt.Sprintf("webhooks/%s/%s/messages/%s", vars["webhook_id"], vars["webhook_token"], vars["message_id"])
		},
	},

	// Gateway
	{
		Method:      "GET",
		PathPattern: EndpointGetGateway,
		RouteID:     "GetGateway",
		URLBuilder: func(vars map[string]string) string {
			return "gateway"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetGatewayBot,
		RouteID:     "GetGatewayBot",
		URLBuilder: func(vars map[string]string) string {
			return "gateway/bot"
		},
	},

	// OAuth2
	{
		Method:      "GET",
		PathPattern: EndpointAuthorizationURL,
		RouteID:     "AuthorizationURL",
		URLBuilder: func(vars map[string]string) string {
			return "oauth2/authorize"
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointTokenURL,
		RouteID:     "TokenURL",
		URLBuilder: func(vars map[string]string) string {
			return "oauth2/token"
		},
	},
	{
		Method:      "POST",
		PathPattern: EndpointTokenRevocationURL,
		RouteID:     "TokenRevocationURL",
		URLBuilder: func(vars map[string]string) string {
			return "oauth2/token/revoke"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentBotApplicationInformation,
		RouteID:     "GetCurrentBotApplicationInformation",
		URLBuilder: func(vars map[string]string) string {
			return "oauth2/applications/@me"
		},
	},
	{
		Method:      "GET",
		PathPattern: EndpointGetCurrentAuthorizationInformation,
		RouteID:     "GetCurrentAuthorizationInformation",
		URLBuilder: func(vars map[string]string) string {
			return "oauth2/@me"
		},
	},
}
