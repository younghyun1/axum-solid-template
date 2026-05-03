CREATE TABLE public.email_verification_questions (
    email_verification_question_id UUID PRIMARY KEY DEFAULT uuidv7(),
    email_verification_question_prompt TEXT NOT NULL,
    email_verification_question_status TEXT NOT NULL DEFAULT 'active',
    email_verification_question_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_question_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_question_deleted_at TIMESTAMPTZ,
    email_verification_question_created_by UUID REFERENCES public.users (user_id),
    email_verification_question_deleted_by UUID REFERENCES public.users (user_id),
    CONSTRAINT email_verification_question_status_check
        CHECK (email_verification_question_status IN ('active', 'deleted'))
);

CREATE INDEX idx_email_verification_questions_status
    ON public.email_verification_questions (email_verification_question_status);
CREATE INDEX idx_email_verification_questions_created_at
    ON public.email_verification_questions (email_verification_question_created_at);

CREATE TABLE public.email_verification_question_answers (
    email_verification_question_answer_id UUID PRIMARY KEY DEFAULT uuidv7(),
    email_verification_question_id UUID NOT NULL,
    email_verification_question_answer_text TEXT NOT NULL,
    email_verification_question_answer_normalized TEXT NOT NULL,
    email_verification_question_answer_status TEXT NOT NULL DEFAULT 'active',
    email_verification_question_answer_created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_question_answer_updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_question_answer_deleted_at TIMESTAMPTZ,
    email_verification_question_answer_created_by UUID REFERENCES public.users (user_id),
    email_verification_question_answer_deleted_by UUID REFERENCES public.users (user_id),
    CONSTRAINT fk_email_verification_question_answers_question
        FOREIGN KEY (email_verification_question_id)
        REFERENCES public.email_verification_questions (email_verification_question_id)
        ON DELETE CASCADE,
    CONSTRAINT email_verification_question_answer_status_check
        CHECK (email_verification_question_answer_status IN ('active', 'deleted'))
);

CREATE UNIQUE INDEX idx_email_verification_question_answers_normalized
    ON public.email_verification_question_answers (
        email_verification_question_id,
        email_verification_question_answer_normalized
    )
    WHERE email_verification_question_answer_status = 'active';
CREATE INDEX idx_email_verification_question_answers_question_id
    ON public.email_verification_question_answers (email_verification_question_id);
CREATE INDEX idx_email_verification_question_answers_status
    ON public.email_verification_question_answers (email_verification_question_answer_status);

CREATE TABLE public.email_verification_questionnaire_state (
    email_verification_questionnaire_state_id UUID PRIMARY KEY DEFAULT uuidv7(),
    email_verification_questionnaire_revision BIGINT NOT NULL DEFAULT 1,
    email_verification_questionnaire_updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO public.email_verification_questionnaire_state (
    email_verification_questionnaire_revision
) VALUES (1);

CREATE TABLE public.email_verification_challenges (
    email_verification_challenge_id UUID PRIMARY KEY DEFAULT uuidv7(),
    email_verification_token_id UUID NOT NULL,
    email_verification_question_id UUID NOT NULL,
    email_verification_challenge_pow_salt TEXT NOT NULL,
    email_verification_challenge_pow_difficulty_bits INTEGER NOT NULL,
    email_verification_challenge_pow_algorithm TEXT NOT NULL,
    email_verification_challenge_minimum_elapsed_ms INTEGER NOT NULL,
    email_verification_challenge_status TEXT NOT NULL DEFAULT 'issued',
    email_verification_challenge_issued_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    email_verification_challenge_expires_at TIMESTAMPTZ NOT NULL,
    email_verification_challenge_solved_at TIMESTAMPTZ,
    email_verification_challenge_failed_at TIMESTAMPTZ,
    email_verification_challenge_attempt_count INTEGER NOT NULL DEFAULT 0,
    email_verification_challenge_last_error TEXT,
    email_verification_challenge_client_ip TEXT,
    email_verification_challenge_user_agent TEXT,
    CONSTRAINT fk_email_verification_challenges_token
        FOREIGN KEY (email_verification_token_id)
        REFERENCES public.email_verification_tokens (email_verification_token_id)
        ON DELETE CASCADE,
    CONSTRAINT fk_email_verification_challenges_question
        FOREIGN KEY (email_verification_question_id)
        REFERENCES public.email_verification_questions (email_verification_question_id),
    CONSTRAINT email_verification_challenge_status_check
        CHECK (
            email_verification_challenge_status IN (
                'issued',
                'solved',
                'failed',
                'expired',
                'superseded'
            )
        )
);

CREATE INDEX idx_email_verification_challenges_token_id
    ON public.email_verification_challenges (email_verification_token_id);
CREATE INDEX idx_email_verification_challenges_question_id
    ON public.email_verification_challenges (email_verification_question_id);
CREATE INDEX idx_email_verification_challenges_status
    ON public.email_verification_challenges (email_verification_challenge_status);
CREATE INDEX idx_email_verification_challenges_expires_at
    ON public.email_verification_challenges (email_verification_challenge_expires_at);

INSERT INTO public.email_verification_questions (
    email_verification_question_id,
    email_verification_question_prompt
) VALUES
    ('019d95b2-8000-7000-8000-000000000001', 'What color is a clear daytime sky?'),
    ('019d95b2-8000-7000-8000-000000000002', 'Type the last word in: rust builds reliable software'),
    ('019d95b2-8000-7000-8000-000000000003', 'What is three plus four?'),
    ('019d95b2-8000-7000-8000-000000000004', 'Which is larger: 9 or 4?'),
    ('019d95b2-8000-7000-8000-000000000005', 'Type the animal word in: table, river, horse, window'),
    ('019d95b2-8000-7000-8000-000000000006', 'What month comes after March?'),
    ('019d95b2-8000-7000-8000-000000000007', 'What is the opposite of cold?'),
    ('019d95b2-8000-7000-8000-000000000008', 'Type the middle number: 2, 5, 8'),
    ('019d95b2-8000-7000-8000-000000000009', 'How many days are in a normal week?'),
    ('019d95b2-8000-7000-8000-00000000000a', 'What letter starts the word template?'),
    ('019d95b2-8000-7000-8000-00000000000b', 'Type the word that means not false.'),
    ('019d95b2-8000-7000-8000-00000000000c', 'What is ten minus six?'),
    ('019d95b2-8000-7000-8000-00000000000d', 'Which word is a color: stone, green, chair?'),
    ('019d95b2-8000-7000-8000-00000000000e', 'Type the second word: secure local account'),
    ('019d95b2-8000-7000-8000-00000000000f', 'What number comes after 11?'),
    ('019d95b2-8000-7000-8000-000000000010', 'What is the opposite of up?'),
    ('019d95b2-8000-7000-8000-000000000011', 'Type the shorter word: authentication or login'),
    ('019d95b2-8000-7000-8000-000000000012', 'How many wheels are on a bicycle?'),
    ('019d95b2-8000-7000-8000-000000000013', 'Which word is not a number: one, two, apple?'),
    ('019d95b2-8000-7000-8000-000000000014', 'Type the final letter of verify.');

INSERT INTO public.email_verification_question_answers (
    email_verification_question_id,
    email_verification_question_answer_text,
    email_verification_question_answer_normalized
) VALUES
    ('019d95b2-8000-7000-8000-000000000001', 'blue', 'blue'),
    ('019d95b2-8000-7000-8000-000000000002', 'software', 'software'),
    ('019d95b2-8000-7000-8000-000000000003', '7', '7'),
    ('019d95b2-8000-7000-8000-000000000003', 'seven', 'seven'),
    ('019d95b2-8000-7000-8000-000000000004', '9', '9'),
    ('019d95b2-8000-7000-8000-000000000004', 'nine', 'nine'),
    ('019d95b2-8000-7000-8000-000000000005', 'horse', 'horse'),
    ('019d95b2-8000-7000-8000-000000000006', 'april', 'april'),
    ('019d95b2-8000-7000-8000-000000000007', 'hot', 'hot'),
    ('019d95b2-8000-7000-8000-000000000008', '5', '5'),
    ('019d95b2-8000-7000-8000-000000000008', 'five', 'five'),
    ('019d95b2-8000-7000-8000-000000000009', '7', '7'),
    ('019d95b2-8000-7000-8000-000000000009', 'seven', 'seven'),
    ('019d95b2-8000-7000-8000-00000000000a', 't', 't'),
    ('019d95b2-8000-7000-8000-00000000000b', 'true', 'true'),
    ('019d95b2-8000-7000-8000-00000000000c', '4', '4'),
    ('019d95b2-8000-7000-8000-00000000000c', 'four', 'four'),
    ('019d95b2-8000-7000-8000-00000000000d', 'green', 'green'),
    ('019d95b2-8000-7000-8000-00000000000e', 'local', 'local'),
    ('019d95b2-8000-7000-8000-00000000000f', '12', '12'),
    ('019d95b2-8000-7000-8000-00000000000f', 'twelve', 'twelve'),
    ('019d95b2-8000-7000-8000-000000000010', 'down', 'down'),
    ('019d95b2-8000-7000-8000-000000000011', 'login', 'login'),
    ('019d95b2-8000-7000-8000-000000000012', '2', '2'),
    ('019d95b2-8000-7000-8000-000000000012', 'two', 'two'),
    ('019d95b2-8000-7000-8000-000000000013', 'apple', 'apple'),
    ('019d95b2-8000-7000-8000-000000000014', 'y', 'y');
