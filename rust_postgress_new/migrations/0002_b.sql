CREATE TABLE public.notification_user (
	id SERIAL PRIMARY KEY,
	user_id int4 NOT NULL,
	email text NOT NULL,
	delivery_type int4 NOT NULL,
	status text DEFAULT 'draft'::text NULL
);