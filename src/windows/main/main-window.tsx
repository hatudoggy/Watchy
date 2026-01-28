import FilterLayout from "@/windows/main/layout/filter-layout/filter-layout";
import ListLayout from "@/windows/main/layout/list-layout/list-layout";
import SearchLayout from "@/windows/main/layout/search-layout/search-layout";
import TopBar from "@/components/layout/top-bar";
import Container from "@/components/ui/container";

export default function MainWindow() {
  return (
    <Container className="flex flex-col overflow-hidden">
      <TopBar />
      <SearchLayout />
      <FilterLayout />
      <ListLayout />
    </Container>
  );
}
