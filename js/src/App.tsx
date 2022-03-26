import React, { useCallback, useState } from 'react';
import { searchWasm } from 'four-fours';

export function App() {
    const [numbers, setNumbers] = useState<string>("");
    const [result, setResult] = useState<string[]>([]);
    const updateResult = useCallback((numbers: string) => {
        if (numbers == "") {
            setResult([]);
        } else {
            const result = searchWasm(numbers);
            setResult([...Array(1001).keys()].map((i) => `${i}: ${result.get(i) ?? "Not Found"}`));
        }
    }, []);

    return <>
        <h1>Rust Wasm four-fours</h1>
        <form onSubmit={(e) => { updateResult(numbers); e.preventDefault(); }}>
            <input type="text" value={numbers} onChange={(x) => setNumbers(x.target.value)} />
            <input type="submit" value="Calc" />
        </form>
        {result.map((result, i) => (<li key={i}>{result}</li>))}
    </>;
}
