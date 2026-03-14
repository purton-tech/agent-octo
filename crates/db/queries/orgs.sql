--: OrgBalance()
--: OrgBillingSummary()

--! get_org_balance : OrgBalance
SELECT
    balance_microcents
FROM org.orgs
WHERE id = public.b64url_to_uuid(:org_id::TEXT);

--! get_org_billing_summary : OrgBillingSummary
SELECT
    name,
    balance_microcents
FROM org.orgs
WHERE id = public.b64url_to_uuid(:org_id::TEXT);
