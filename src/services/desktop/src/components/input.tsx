import "../Global.css";

type Props = {
    label?: string;
    placeholder?: string;
    type?: string;
};

export default function InputField({label, placeholder, type}: Props) {
    return (
        <div className="input-container">
            {label && (
                <label className="input-label">
                    {label}
                </label>
            )}
            <input
                type={type}
                placeholder={placeholder}
                className="input-field"
            />
        </div>
    )
}
