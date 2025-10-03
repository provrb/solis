import Header from "../components/Header";

function EngineerPanel() {
  return (
    <div className="h-full bg-slate-50 overflow-y-auto">
      <Header
        title="Race Engineer"
        subtitle="Ask questions that will be answered using your telemetry"
      />
    </div>
  );
}

export default EngineerPanel;
