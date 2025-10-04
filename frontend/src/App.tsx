import {useState} from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import Login from "./pages/Login.tsx";
import Protected from "./pages/Protected.tsx";

async function typedFetch<T>(url: string, options?: RequestInit): Promise<T> {
    const response = await fetch(url, options);

    console.log(response)

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    return data as T;
}

interface Post {
    id: string;
    title: string;
}

async function listPosts(limit: number): Promise<Post[]> {
    return typedFetch<Post[]>(`http://0.0.0.0:7878/posts?limit=${limit}`);
}

// async function createProduct(productData: Omit<Product, 'id'>): Promise<Product> {
//     return typedFetch<Product>('/api/products', {
//         method: 'POST',
//         headers: {
//             'Content-Type': 'application/json',
//         },
//         body: JSON.stringify(productData),
//     });
// }

function App() {

    console.log(`VITE_SUBWAY_AUTH_MODE == ${import.meta.env.VITE_SUBWAY_AUTH_MODE}`)

    const path = window.location.pathname;

    if (path === '/login') {
        return <Login/>;
    }

    if (path === '/protected') {
        return <Protected/>;
    }

    const [count, setCount] = useState(0)
    const [isLoading, setIsLoading] = useState(false);
    const [data, setData] = useState<Post[]>([]);

    const fetchData = async () => {
        setIsLoading(true); // Set loading state to true

        try {
            const thing = await listPosts(10)
            setData(thing); // Update data state
        } finally {
            setIsLoading(false); // Set loading state back to false
        }
    };

    return (
        <>
            <div>
                <a href="https://vite.dev" target="_blank">
                    <img src={viteLogo} className="logo" alt="Vite logo"/>
                </a>
                <a href="https://react.dev" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo"/>
                </a>
            </div>
            <h1>Vite + React</h1>
            <div className="card">
                <button onClick={() => setCount((count) => count + 1)}>
                    count is {count}
                </button>
                <div>
                    <button onClick={fetchData} disabled={isLoading}>
                        {isLoading ? 'Loading...' : 'Fetch Data'}
                    </button>

                    {data && <p>{JSON.stringify(data)}</p>}
                </div>
                <p>
                    Edit <code>src/App.tsx</code> and save to test HMR
                </p>
            </div>
            <p className="read-the-docs">
                Click on the Vite and React logos to learn more
            </p>
        </>
    )
}

export default App
