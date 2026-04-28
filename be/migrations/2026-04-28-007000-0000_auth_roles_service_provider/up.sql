UPDATE public.roles
SET
    role_name = 'admin',
    role_description = 'Administrator role with total access; owner of the site.'
WHERE role_id = '019a6c86-8bca-7b91-b9c0-1d4cc96b3263';

INSERT INTO public.roles (role_id, role_name, role_description)
VALUES (
    '019dd245-98e6-7b57-ade5-92019a275c6e',
    'service_provider',
    'Service provider client role for serving user clients'
)
ON CONFLICT (role_id) DO UPDATE
SET
    role_name = EXCLUDED.role_name,
    role_description = EXCLUDED.role_description;

UPDATE public.roles
SET role_description = 'Client user role for requesting services'
WHERE role_id = '019a6c86-bfa6-7903-9176-dc5f66f729fe';
