<?php

declare(strict_types=1);

namespace Psl\Internal;

use Psl;
use Psl\Type;
use Psl\Exception;

use function in_array;
use function mb_internal_encoding;
use function mb_list_encodings;

/**
 * @psalm-pure
 */
function is_encoding_valid(string $encoding): bool
{
    /** @psalm-suppress ImpureFunctionCall */
    return in_array($encoding, mb_list_encodings(), true);
}