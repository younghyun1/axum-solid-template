DELETE FROM public.roles
WHERE role_id = '019dd245-98e6-7b57-ade5-92019a275c6e';

UPDATE public.roles
SET
    role_name = 'younghyun',
    role_description = 'Administrator role with total access; owner of the site.'
WHERE role_id = '019a6c86-8bca-7b91-b9c0-1d4cc96b3263';

UPDATE public.roles
SET role_description = 'Regular user role with limited access'
WHERE role_id = '019a6c86-bfa6-7903-9176-dc5f66f729fe';
