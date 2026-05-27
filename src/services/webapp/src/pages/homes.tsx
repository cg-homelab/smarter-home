import * as React from "react";
import {
  Home,
  Plus,
  MapPin,
  Key,
  AlertCircle,
  Loader2,
  Pencil,
  Check,
  X,
} from "lucide-react";
import {
  homeService,
  type Home as HomeModel,
  type ApiError,
} from "@/services/home";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

interface FormState {
  name: string;
  address: string;
}

const EMPTY_FORM: FormState = { name: "", address: "" };

export function Homes() {
  const [homes, setHomes] = React.useState<HomeModel[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  // Create form
  const [showForm, setShowForm] = React.useState(false);
  const [form, setForm] = React.useState<FormState>(EMPTY_FORM);
  const [formError, setFormError] = React.useState<string | null>(null);
  const [submitting, setSubmitting] = React.useState(false);

  // Edit state
  const [editingId, setEditingId] = React.useState<string | null>(null);
  const [editForm, setEditForm] = React.useState<FormState>(EMPTY_FORM);
  const [editError, setEditError] = React.useState<string | null>(null);
  const [editSubmitting, setEditSubmitting] = React.useState(false);

  const [revealedTokens, setRevealedTokens] = React.useState<Set<string>>(
    new Set(),
  );

  React.useEffect(() => {
    homeService
      .getHomes()
      .then(setHomes)
      .catch((err: ApiError) => setError(err.message ?? "Failed to load homes"))
      .finally(() => setLoading(false));
  }, []);

  function handleFieldChange(e: React.ChangeEvent<HTMLInputElement>) {
    const { name, value } = e.target;
    setForm((prev) => ({ ...prev, [name]: value }));
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setFormError(null);
    setSubmitting(true);
    try {
      const created = await homeService.createHome({
        name: form.name,
        address: form.address,
      });
      setHomes((prev) => [...prev, created]);
      setForm(EMPTY_FORM);
      setShowForm(false);
    } catch (err) {
      const apiErr = err as ApiError;
      setFormError(apiErr.message ?? "Failed to create home");
    } finally {
      setSubmitting(false);
    }
  }

  function startEdit(home: HomeModel) {
    setEditingId(home.id);
    setEditForm({ name: home.name, address: home.address });
    setEditError(null);
  }

  function cancelEdit() {
    setEditingId(null);
    setEditForm(EMPTY_FORM);
    setEditError(null);
  }

  function handleEditFieldChange(e: React.ChangeEvent<HTMLInputElement>) {
    const { name, value } = e.target;
    setEditForm((prev) => ({ ...prev, [name]: value }));
  }

  async function handleEditSubmit(e: React.FormEvent, id: string) {
    e.preventDefault();
    setEditError(null);
    setEditSubmitting(true);
    try {
      const updated = await homeService.updateHome(id, {
        name: editForm.name,
        address: editForm.address,
      });
      setHomes((prev) => prev.map((h) => (h.id === id ? updated : h)));
      cancelEdit();
    } catch (err) {
      const apiErr = err as ApiError;
      setEditError(apiErr.message ?? "Failed to update home");
    } finally {
      setEditSubmitting(false);
    }
  }

  function toggleToken(id: string) {
    setRevealedTokens((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  }

  function handleCancel() {
    setForm(EMPTY_FORM);
    setFormError(null);
    setShowForm(false);
  }

  return (
    <div className="flex flex-col gap-6 p-4 sm:p-6 lg:p-8">
      {/* Page header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div className="flex flex-col gap-1">
          <h1 className="text-lg font-semibold text-gray-900 dark:text-gray-100">
            Homes
          </h1>
          <p className="text-xs text-gray-500 dark:text-gray-400">
            Manage your registered homes and their access tokens
          </p>
        </div>
        {!showForm && (
          <Button size="sm" onClick={() => setShowForm(true)}>
            <Plus className="mr-1.5 h-4 w-4" />
            Add home
          </Button>
        )}
      </div>

      {/* Create home form */}
      {showForm && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">New home</CardTitle>
            <CardDescription>
              Fill in the details for your new home.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <form onSubmit={handleSubmit} className="flex flex-col gap-4">
              <div className="grid gap-4 sm:grid-cols-2">
                <div className="flex flex-col gap-1.5">
                  <Label htmlFor="name">Name</Label>
                  <Input
                    id="name"
                    name="name"
                    placeholder="e.g. My House"
                    value={form.name}
                    onChange={handleFieldChange}
                    required
                    disabled={submitting}
                  />
                </div>
                <div className="flex flex-col gap-1.5">
                  <Label htmlFor="address">Address</Label>
                  <Input
                    id="address"
                    name="address"
                    placeholder="e.g. 123 Main St"
                    value={form.address}
                    onChange={handleFieldChange}
                    required
                    disabled={submitting}
                  />
                </div>
              </div>

              {formError && (
                <div className="flex items-center gap-2 rounded-md border border-destructive/40 bg-destructive/10 px-3 py-2 text-sm text-destructive">
                  <AlertCircle className="h-4 w-4 shrink-0" />
                  {formError}
                </div>
              )}

              <div className="flex justify-end gap-2">
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={handleCancel}
                  disabled={submitting}
                >
                  Cancel
                </Button>
                <Button type="submit" size="sm" disabled={submitting}>
                  {submitting ? (
                    <>
                      <Loader2 className="mr-1.5 h-4 w-4 animate-spin" />
                      Creating…
                    </>
                  ) : (
                    "Create home"
                  )}
                </Button>
              </div>
            </form>
          </CardContent>
        </Card>
      )}

      {/* Loading state */}
      {loading && (
        <div className="flex items-center justify-center py-16 text-muted-foreground">
          <Loader2 className="mr-2 h-5 w-5 animate-spin" />
          Loading homes…
        </div>
      )}

      {/* Fetch error */}
      {!loading && error && (
        <div className="flex items-center gap-2 rounded-md border border-destructive/40 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          <AlertCircle className="h-4 w-4 shrink-0" />
          {error}
        </div>
      )}

      {/* Empty state */}
      {!loading && !error && homes.length === 0 && (
        <div className="flex flex-col items-center justify-center gap-3 rounded-xl border border-dashed py-16 text-center text-muted-foreground">
          <Home className="h-10 w-10 opacity-40" />
          <div>
            <p className="text-sm font-medium">No homes yet</p>
            <p className="text-xs">
              Add your first home to start monitoring energy usage.
            </p>
          </div>
          {!showForm && (
            <Button
              size="sm"
              variant="secondary"
              onClick={() => setShowForm(true)}
            >
              <Plus className="mr-1.5 h-4 w-4" />
              Add home
            </Button>
          )}
        </div>
      )}

      {/* Homes grid */}
      {!loading && !error && homes.length > 0 && (
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          {homes.map((home) => {
            const isEditing = editingId === home.id;
            const isRevealed = revealedTokens.has(home.id);

            return (
              <Card key={home.id}>
                {isEditing ? (
                  /* ── Edit mode ── */
                  <>
                    <CardHeader>
                      <CardTitle className="text-base">Edit home</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <form
                        onSubmit={(e) => handleEditSubmit(e, home.id)}
                        className="flex flex-col gap-3"
                      >
                        <div className="flex flex-col gap-1.5">
                          <Label htmlFor={`edit-name-${home.id}`}>Name</Label>
                          <Input
                            id={`edit-name-${home.id}`}
                            name="name"
                            value={editForm.name}
                            onChange={handleEditFieldChange}
                            required
                            disabled={editSubmitting}
                            autoFocus
                          />
                        </div>
                        <div className="flex flex-col gap-1.5">
                          <Label htmlFor={`edit-address-${home.id}`}>
                            Address
                          </Label>
                          <Input
                            id={`edit-address-${home.id}`}
                            name="address"
                            value={editForm.address}
                            onChange={handleEditFieldChange}
                            required
                            disabled={editSubmitting}
                          />
                        </div>

                        {editError && (
                          <div className="flex items-center gap-2 rounded-md border border-destructive/40 bg-destructive/10 px-3 py-2 text-sm text-destructive">
                            <AlertCircle className="h-4 w-4 shrink-0" />
                            {editError}
                          </div>
                        )}

                        <div className="flex justify-end gap-2 pt-1">
                          <Button
                            type="button"
                            variant="ghost"
                            size="sm"
                            onClick={cancelEdit}
                            disabled={editSubmitting}
                          >
                            <X className="h-4 w-4" />
                            Cancel
                          </Button>
                          <Button
                            type="submit"
                            size="sm"
                            disabled={editSubmitting}
                          >
                            {editSubmitting ? (
                              <Loader2 className="h-4 w-4 animate-spin" />
                            ) : (
                              <Check className="h-4 w-4" />
                            )}
                            Save
                          </Button>
                        </div>
                      </form>
                    </CardContent>
                  </>
                ) : (
                  /* ── View mode ── */
                  <>
                    <CardHeader>
                      <CardTitle className="flex items-center justify-between gap-2 text-base">
                        <span className="flex items-center gap-2">
                          <Home className="h-4 w-4 text-muted-foreground" />
                          {home.name}
                        </span>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
                          onClick={() => startEdit(home)}
                          aria-label="Edit home"
                        >
                          <Pencil className="h-3.5 w-3.5" />
                        </Button>
                      </CardTitle>
                    </CardHeader>
                    <CardContent className="flex flex-col gap-3">
                      <div className="flex items-start gap-2 text-sm text-muted-foreground">
                        <MapPin className="mt-0.5 h-4 w-4 shrink-0" />
                        <span>{home.address}</span>
                      </div>
                      <div className="flex items-center gap-2 text-sm">
                        <Key className="h-4 w-4 shrink-0 text-muted-foreground" />
                        <code className="flex-1 truncate rounded bg-muted px-1.5 py-0.5 text-xs font-mono">
                          {isRevealed
                            ? home.writeToken
                            : "••••••••••••••••••••"}
                        </code>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-6 px-2 text-xs"
                          onClick={() => toggleToken(home.id)}
                        >
                          {isRevealed ? "Hide" : "Show"}
                        </Button>
                      </div>
                    </CardContent>
                  </>
                )}
              </Card>
            );
          })}
        </div>
      )}
    </div>
  );
}
