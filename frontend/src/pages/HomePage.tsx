import {useState} from 'react'
import './Home.css'
import {Link} from "react-router-dom";
import realKeycloak from "../auth/keycloak.ts";
import fakeKeycloak from "../auth/fakeKeycloak.ts";

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

function Home() {

    console.log(`VITE_SUBWAY_AUTH_MODE == ${import.meta.env.VITE_SUBWAY_AUTH_MODE}`)

    const useRealAuth = import.meta.env.VITE_SUBWAY_AUTH_MODE === 'docker';
    const keycloak = useRealAuth ? realKeycloak : fakeKeycloak;

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

    const username: string | null = keycloak.tokenParsed?.preferred_username

    return (
        <>
            { keycloak.authenticated
                ?
                <div>
                    <button onClick={() => keycloak.logout({ redirectUri: window.location.origin })}>
                        Logout
                    </button>
                </div>
                :
                <div>
                    <Link to={'/login'}>
                        <button>
                            Log in
                        </button>
                    </Link>
                </div>
            }
            <div>
                <Link to={'/user-only'}>
                    <button>
                        User-Only Page
                    </button>
                </Link>
            </div>
            <div>
                <Link to={'/admin-only'}>
                    <button>
                        Admin-Only Page
                    </button>
                </Link>
            </div>
            <p>Welcome{username ? `, ${username}` : '' }!</p>
            <h1>Vite + React</h1>
            <div className="card">
                <div>
                    <button onClick={fetchData} disabled={isLoading}>
                        {isLoading ? 'Loading...' : 'Fetch Data'}
                    </button>

                    {data && <p>{JSON.stringify(data)}</p>}
                </div>
            </div>
        </>
    )
}

export default Home
