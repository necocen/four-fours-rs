import React, { memo, useState } from 'react';
import * as Comlink from 'comlink';
const fourFours = Comlink.wrap(new Worker(new URL('wasm.ts', import.meta.url), { type: 'module' })) as any;
const search = await fourFours();

export const App = memo(() => {
    const [numbers, setNumbers] = useState<string>("");
    const [result, setResult] = useState<string[]>([]);
    const [isLoading, setIsLoading] = useState(false);
    const updateResult = async (numbers: string) => {
        setIsLoading(true);
        if (numbers == "") {
            setIsLoading(false);
            setResult([]);
        } else {
            setResult([]);
            try {
                const result = await search(numbers);
                setResult([...Array(1001).keys()].map((i) => (result.get(i) ? `${i} = ${result.get(i)}` : undefined)).filter((s) => !!s).map((s) => s as string));
            } catch (error: any) {
                console.error(error.toString());
                setResult([]);
            } finally {
                setIsLoading(false);
            }
        }
    };

    return <>
        <h1>Rust Wasm four-fours</h1>
        <form onSubmit={(e) => { updateResult(numbers); e.preventDefault(); }}>
            <input type="text" value={numbers} onChange={(x) => setNumbers(x.target.value)} />
            <input type="submit" value="Calc" />
        </form>
        <ul>
            {result.map((result, i) => (<li key={i}>{result}</li>))}
        </ul>
        {isLoading && "Loading..."}
    </>;
});
