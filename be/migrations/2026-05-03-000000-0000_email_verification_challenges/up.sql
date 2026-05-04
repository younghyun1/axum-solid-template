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

WITH seeded_questions (email_verification_question_prompt) AS (
    VALUES
        ('What color is a clear daytime sky?'),
        ('Type the last word in: rust builds reliable software'),
        ('What is three plus four?'),
        ('Which is larger: 9 or 4?'),
        ('Type the animal word in: table, river, horse, window'),
        ('What month comes after March?'),
        ('What is the opposite of cold?'),
        ('Type the middle number: 2, 5, 8'),
        ('How many days are in a normal week?'),
        ('What letter starts the word template?'),
        ('Type the word that means not false.'),
        ('What is ten minus six?'),
        ('Which word is a color: stone, green, chair?'),
        ('Type the second word: secure local account'),
        ('What number comes after 11?'),
        ('What is the opposite of up?'),
        ('Type the shorter word: authentication or login'),
        ('How many wheels are on a bicycle?'),
        ('Which word is not a number: one, two, apple?'),
        ('Type the final letter of verify.')
),
inserted_questions AS (
    INSERT INTO public.email_verification_questions (
        email_verification_question_prompt
    )
    SELECT email_verification_question_prompt
    FROM seeded_questions
    RETURNING
        email_verification_question_id,
        email_verification_question_prompt
),
seeded_answers (
    email_verification_question_prompt,
    email_verification_question_answer_text,
    email_verification_question_answer_normalized
) AS (
    VALUES
        ('What color is a clear daytime sky?', 'blue', 'blue'),
        ('Type the last word in: rust builds reliable software', 'software', 'software'),
        ('What is three plus four?', '7', '7'),
        ('What is three plus four?', 'seven', 'seven'),
        ('Which is larger: 9 or 4?', '9', '9'),
        ('Which is larger: 9 or 4?', 'nine', 'nine'),
        ('Type the animal word in: table, river, horse, window', 'horse', 'horse'),
        ('What month comes after March?', 'april', 'april'),
        ('What is the opposite of cold?', 'hot', 'hot'),
        ('Type the middle number: 2, 5, 8', '5', '5'),
        ('Type the middle number: 2, 5, 8', 'five', 'five'),
        ('How many days are in a normal week?', '7', '7'),
        ('How many days are in a normal week?', 'seven', 'seven'),
        ('What letter starts the word template?', 't', 't'),
        ('Type the word that means not false.', 'true', 'true'),
        ('What is ten minus six?', '4', '4'),
        ('What is ten minus six?', 'four', 'four'),
        ('Which word is a color: stone, green, chair?', 'green', 'green'),
        ('Type the second word: secure local account', 'local', 'local'),
        ('What number comes after 11?', '12', '12'),
        ('What number comes after 11?', 'twelve', 'twelve'),
        ('What is the opposite of up?', 'down', 'down'),
        ('Type the shorter word: authentication or login', 'login', 'login'),
        ('How many wheels are on a bicycle?', '2', '2'),
        ('How many wheels are on a bicycle?', 'two', 'two'),
        ('Which word is not a number: one, two, apple?', 'apple', 'apple'),
        ('Type the final letter of verify.', 'y', 'y')
)
INSERT INTO public.email_verification_question_answers (
    email_verification_question_id,
    email_verification_question_answer_text,
    email_verification_question_answer_normalized
)
SELECT
    inserted_questions.email_verification_question_id,
    seeded_answers.email_verification_question_answer_text,
    seeded_answers.email_verification_question_answer_normalized
FROM seeded_answers
JOIN inserted_questions
    ON inserted_questions.email_verification_question_prompt =
        seeded_answers.email_verification_question_prompt;
