import { ComposingNormalizer } from "icu4x"
export function normalize(s) {
    return (function (...args) { return args[0].normalize(...args.slice(1)) }).apply(
        null,
        [
            ComposingNormalizer.createNfc.apply(
                null,
                [
                ]
            ),
            s
        ]
    );
}
