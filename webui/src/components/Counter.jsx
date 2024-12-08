import {useEffect, useState} from "preact/hooks";

export default function Counter({onChange}) {
    const [count, setCount] = useState(0);

    useEffect(() => {
        if (onChange) {
            onChange(count);
        }
    }, [count]);
    return (<div class="btn-group">
        <button class="btn btn-secondary"
                onClick={() => setCount(count - 1)}
        >-
        </button>
        <button class="btn" contentEditable={true}>{count}</button>
        <button class="btn btn-secondary"
                onClick={() => setCount(count + 1)}
        >+
        </button>
    </div>);
}