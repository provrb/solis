type HeaderProp = {
  title: string;
  subtitle?: string | null;
};

const Header: React.FC<HeaderProp> = ({ title, subtitle }) => {
  return (
    <div className="px-6 pt-6 pb-3">
      <h1 className="text-2xl font-bold text-black font-montserrat">{title}</h1>
      {subtitle && (
        <p className="text-slate-600 font-montserrat mt-1">{subtitle}</p>
      )}
    </div>
  );
};

export default Header;
