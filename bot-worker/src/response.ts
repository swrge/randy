import { InteractionResponseTypes } from '@discordeno/types';

export class JsonResponse extends Response {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(body: any, init?: ResponseInit) {
    const jsonBody = JSON.stringify(body);
    init = init || {
      headers: {
        'Content-Type': 'application/json',
      },
    };
    super(jsonBody, init);
  }
}

export async function BadRequest(message: any): Promise<Response> {
  return new JsonResponse({ error: message }, { status: 400 });
}

export async function Unauthorized(message: string): Promise<Response> {
  return new JsonResponse({ error: message }, { status: 401 });
}

export async function Forbidden(message: string): Promise<Response> {
  return new JsonResponse({ error: message }, { status: 403 });
}

export async function NotFound(message: string): Promise<Response> {
  return new JsonResponse({ error: message }, { status: 404 });
}

export async function UnknownInteraction(): Promise<Response> {
  return BadRequest({
    type: InteractionResponseTypes.ChannelMessageWithSource,
    data: {
      content: 'Unknown interaction type',
    },
  });
}

export async function Pong(): Promise<Response> {
  return new JsonResponse({ type: InteractionResponseTypes.Pong });
}
