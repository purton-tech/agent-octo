--: IntegrationConnectionCard()
--: ConnectableIntegration()
--: IntegrationAuthRequirement()
--: IntegrationConnectionMutation()

--! list_integration_connections : IntegrationConnectionCard
SELECT
    ic.id,
    ic.integration_id,
    ic.name,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS integration_name,
    ic.auth_type::TEXT AS auth_type,
    ic.visibility::TEXT AS visibility,
    ic.updated_at
FROM public.integration_connections ic
JOIN public.integrations i
  ON i.id = ic.integration_id
WHERE ic.org_id = public.b64url_to_uuid(:org_id::TEXT)
  AND (
      ic.visibility = 'org'
      OR ic.created_by_user_id = auth.uid()
  )
ORDER BY ic.updated_at DESC;

--! list_connectable_integrations : ConnectableIntegration
SELECT
    i.id,
    COALESCE(i.openapi_spec #>> '{info,title}', 'Untitled') AS name,
    COALESCE(i.openapi_spec #>> '{info,description}', '') AS description,
    (
        jsonb_path_exists(i.openapi_spec, '$.security[*]')
        OR jsonb_path_exists(i.openapi_spec, '$.components.securitySchemes.*')
    ) AS requires_auth
FROM public.integrations i
WHERE i.org_id = public.b64url_to_uuid(:org_id::TEXT)
  AND (
      i.visibility = 'org'
      OR i.created_by_user_id = auth.uid()
  )
ORDER BY name ASC;

--! get_integration_auth_requirement : IntegrationAuthRequirement
SELECT
    i.id AS integration_id,
    (
        jsonb_path_exists(i.openapi_spec, '$.security[*]')
        OR jsonb_path_exists(i.openapi_spec, '$.components.securitySchemes.*')
    ) AS requires_auth
FROM public.integrations i
WHERE i.id = :integration_id::UUID
  AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
LIMIT 1;

--! create_integration_connection : IntegrationConnectionMutation
WITH inserted AS (
    INSERT INTO public.integration_connections (
        org_id,
        integration_id,
        created_by_user_id,
        visibility,
        name,
        auth_type,
        api_key_secret_ref
    )
    SELECT
        public.b64url_to_uuid(:org_id::TEXT),
        :integration_id::UUID,
        auth.uid(),
        :visibility::resource_visibility,
        COALESCE(i.openapi_spec #>> '{info,title}', 'Integration') || ' connection',
        'api_key'::integration_auth_type,
        :api_key_secret_ref::TEXT
    FROM public.integrations i
    WHERE i.id = :integration_id::UUID
      AND i.org_id = public.b64url_to_uuid(:org_id::TEXT)
    RETURNING id
)
SELECT EXISTS(SELECT 1 FROM inserted) AS changed;
