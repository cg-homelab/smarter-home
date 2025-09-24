import "../Global.css";

type Props = {
    name: string;
    email: string;
};

export default function UserCard({ name, email }: Props) {
    return (
        <div className="user-card">
            <div className="user-info">
                <h2 className="user-name">{name}</h2>
                <p className="user-email">{email}</p>
            </div>
        </div> 
    )
}