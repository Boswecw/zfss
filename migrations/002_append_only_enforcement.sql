-- Enforce append-only semantics on canonical ZFSS tables.
-- Trigger-based enforcement ensures any UPDATE or DELETE raises an explicit error telling operators to append instead.

CREATE OR REPLACE FUNCTION zfss_forbid_mutation()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    RAISE EXCEPTION 'ZFSS doctrine violation: updates/deletes forbidden on %', TG_TABLE_NAME;
END;
$$;

CREATE TRIGGER trg_issues_prevent_mutation
BEFORE UPDATE OR DELETE ON issues
FOR EACH ROW EXECUTE FUNCTION zfss_forbid_mutation();

CREATE TRIGGER trg_signals_prevent_mutation
BEFORE UPDATE OR DELETE ON signals
FOR EACH ROW EXECUTE FUNCTION zfss_forbid_mutation();

CREATE TRIGGER trg_decisions_prevent_mutation
BEFORE UPDATE OR DELETE ON decisions
FOR EACH ROW EXECUTE FUNCTION zfss_forbid_mutation();

CREATE TRIGGER trg_artifacts_prevent_mutation
BEFORE UPDATE OR DELETE ON artifacts
FOR EACH ROW EXECUTE FUNCTION zfss_forbid_mutation();

CREATE TRIGGER trg_responses_prevent_mutation
BEFORE UPDATE OR DELETE ON responses
FOR EACH ROW EXECUTE FUNCTION zfss_forbid_mutation();
