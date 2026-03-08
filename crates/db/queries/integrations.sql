--: IntegrationCard()
--: IntegrationForm()
--: IntegrationMutation()

--! list_integrations : IntegrationCard
SELECT
    i.id,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.visibility::TEXT AS visibility,
    i.updated_at
FROM public.integrations i
WHERE i.org_id = public.b64url_to_uuid(:org_id::TEXT)
  AND (
      i.visibility = 'org'
      OR i.created_by_user_id = auth.uid()
  )
ORDER BY i.updated_at DESC;

--! get_integration_for_edit : IntegrationForm
SELECT
    i.id,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    i.visibility::TEXT AS visibility,
    i.openapi_spec::TEXT AS openapi_spec
FROM public.integrations i
WHERE i.id = :id::UUID
  AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
LIMIT 1;

--! create_integration : IntegrationMutation
WITH inserted AS (
    INSERT INTO public.integrations (
        org_id,
        created_by_user_id,
        visibility,
        openapi_spec
    )
    VALUES (
        public.b64url_to_uuid(:org_id::TEXT),
        auth.uid(),
        :visibility::resource_visibility,
        :openapi_spec::JSONB
    )
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM inserted) AS changed;

--! update_integration : IntegrationMutation
WITH updated AS (
    UPDATE public.integrations i
    SET
        visibility = :visibility::resource_visibility,
        openapi_spec = :openapi_spec::JSONB,
        updated_at = NOW()
    WHERE i.id = :id::UUID
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM updated) AS changed;

--! delete_integration : IntegrationMutation
WITH deleted AS (
    DELETE FROM public.integrations i
    WHERE i.id = :id::UUID
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM deleted) AS changed;
