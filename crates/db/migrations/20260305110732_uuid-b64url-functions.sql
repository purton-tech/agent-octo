-- migrate:up
CREATE OR REPLACE FUNCTION uuid_to_b64url(u uuid)
RETURNS text
LANGUAGE sql
IMMUTABLE
AS $$
SELECT rtrim(
         translate(
           encode(decode(replace(u::text, '-', ''), 'hex'), 'base64'),
           '+/', '-_'
         ),
         '='
       );
$$;

CREATE OR REPLACE FUNCTION b64url_to_uuid(s text)
RETURNS uuid
LANGUAGE sql
IMMUTABLE
AS $$
SELECT (
  encode(
    decode(
      translate(
        s || repeat('=', (4 - length(s) % 4) % 4),
        '-_',
        '+/'
      ),
      'base64'
    ),
    'hex'
  )
)::uuid;
$$;

GRANT EXECUTE ON FUNCTION public.uuid_to_b64url(UUID) TO application_user;
GRANT EXECUTE ON FUNCTION public.uuid_to_b64url(UUID) TO application_readonly;
GRANT EXECUTE ON FUNCTION public.b64url_to_uuid(TEXT) TO application_user;
GRANT EXECUTE ON FUNCTION public.b64url_to_uuid(TEXT) TO application_readonly;

-- migrate:down
DROP FUNCTION IF EXISTS public.b64url_to_uuid(TEXT);
DROP FUNCTION IF EXISTS public.uuid_to_b64url(UUID);
