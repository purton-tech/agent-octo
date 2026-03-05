--: ChannelCard()

--! list_org_channels : ChannelCard
SELECT
    id,
    name,
    kind::TEXT AS kind,
    visibility::TEXT AS visibility,
    updated_at
FROM public.channels
WHERE org_id = public.b64url_to_uuid(:org_id::TEXT)
  AND (
      visibility = 'org'
      OR created_by_user_id = auth.uid()
  )
ORDER BY updated_at DESC;
