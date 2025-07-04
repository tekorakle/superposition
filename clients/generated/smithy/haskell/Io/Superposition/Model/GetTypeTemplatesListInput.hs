module Io.Superposition.Model.GetTypeTemplatesListInput (
    setCount,
    setPage,
    setAll',
    setWorkspaceId,
    setOrgId,
    build,
    GetTypeTemplatesListInputBuilder,
    GetTypeTemplatesListInput,
    count,
    page,
    all',
    workspace_id,
    org_id
) where
import qualified Control.Applicative
import qualified Control.Monad
import qualified Data.Aeson
import qualified Data.Either
import qualified Data.Eq
import qualified Data.Functor
import qualified Data.Maybe
import qualified Data.Text
import qualified GHC.Generics
import qualified GHC.Show

data GetTypeTemplatesListInput = GetTypeTemplatesListInput {
    count :: Data.Maybe.Maybe Integer,
    page :: Data.Maybe.Maybe Integer,
    all' :: Data.Maybe.Maybe Bool,
    workspace_id :: Data.Text.Text,
    org_id :: Data.Text.Text
} deriving (
  GHC.Show.Show,
  Data.Eq.Eq,
  GHC.Generics.Generic
  )

instance Data.Aeson.ToJSON GetTypeTemplatesListInput where
    toJSON a = Data.Aeson.object [
        "count" Data.Aeson..= count a,
        "page" Data.Aeson..= page a,
        "all" Data.Aeson..= all' a,
        "workspace_id" Data.Aeson..= workspace_id a,
        "org_id" Data.Aeson..= org_id a
        ]
    


instance Data.Aeson.FromJSON GetTypeTemplatesListInput where
    parseJSON = Data.Aeson.withObject "GetTypeTemplatesListInput" $ \v -> GetTypeTemplatesListInput
        Data.Functor.<$> (v Data.Aeson..: "count")
        Control.Applicative.<*> (v Data.Aeson..: "page")
        Control.Applicative.<*> (v Data.Aeson..: "all")
        Control.Applicative.<*> (v Data.Aeson..: "workspace_id")
        Control.Applicative.<*> (v Data.Aeson..: "org_id")
    



data GetTypeTemplatesListInputBuilderState = GetTypeTemplatesListInputBuilderState {
    countBuilderState :: Data.Maybe.Maybe Integer,
    pageBuilderState :: Data.Maybe.Maybe Integer,
    all'BuilderState :: Data.Maybe.Maybe Bool,
    workspace_idBuilderState :: Data.Maybe.Maybe Data.Text.Text,
    org_idBuilderState :: Data.Maybe.Maybe Data.Text.Text
} deriving (
  GHC.Generics.Generic
  )

defaultBuilderState :: GetTypeTemplatesListInputBuilderState
defaultBuilderState = GetTypeTemplatesListInputBuilderState {
    countBuilderState = Data.Maybe.Nothing,
    pageBuilderState = Data.Maybe.Nothing,
    all'BuilderState = Data.Maybe.Nothing,
    workspace_idBuilderState = Data.Maybe.Nothing,
    org_idBuilderState = Data.Maybe.Nothing
}

newtype GetTypeTemplatesListInputBuilder a = GetTypeTemplatesListInputBuilder {
    runGetTypeTemplatesListInputBuilder :: GetTypeTemplatesListInputBuilderState -> (GetTypeTemplatesListInputBuilderState, a)
}

instance Data.Functor.Functor GetTypeTemplatesListInputBuilder where
    fmap f (GetTypeTemplatesListInputBuilder g) =
        GetTypeTemplatesListInputBuilder (\s -> let (s', a) = g s in (s', f a))

instance Control.Applicative.Applicative GetTypeTemplatesListInputBuilder where
    pure a = GetTypeTemplatesListInputBuilder (\s -> (s, a))
    (GetTypeTemplatesListInputBuilder f) <*> (GetTypeTemplatesListInputBuilder g) = GetTypeTemplatesListInputBuilder (\s ->
        let (s', h) = f s
            (s'', a) = g s'
        in (s'', h a))

instance Control.Monad.Monad GetTypeTemplatesListInputBuilder where
    (GetTypeTemplatesListInputBuilder f) >>= g = GetTypeTemplatesListInputBuilder (\s ->
        let (s', a) = f s
            (GetTypeTemplatesListInputBuilder h) = g a
        in h s')

setCount :: Data.Maybe.Maybe Integer -> GetTypeTemplatesListInputBuilder ()
setCount value =
   GetTypeTemplatesListInputBuilder (\s -> (s { countBuilderState = value }, ()))

setPage :: Data.Maybe.Maybe Integer -> GetTypeTemplatesListInputBuilder ()
setPage value =
   GetTypeTemplatesListInputBuilder (\s -> (s { pageBuilderState = value }, ()))

setAll' :: Data.Maybe.Maybe Bool -> GetTypeTemplatesListInputBuilder ()
setAll' value =
   GetTypeTemplatesListInputBuilder (\s -> (s { all'BuilderState = value }, ()))

setWorkspaceId :: Data.Text.Text -> GetTypeTemplatesListInputBuilder ()
setWorkspaceId value =
   GetTypeTemplatesListInputBuilder (\s -> (s { workspace_idBuilderState = Data.Maybe.Just value }, ()))

setOrgId :: Data.Text.Text -> GetTypeTemplatesListInputBuilder ()
setOrgId value =
   GetTypeTemplatesListInputBuilder (\s -> (s { org_idBuilderState = Data.Maybe.Just value }, ()))

build :: GetTypeTemplatesListInputBuilder () -> Data.Either.Either Data.Text.Text GetTypeTemplatesListInput
build builder = do
    let (st, _) = runGetTypeTemplatesListInputBuilder builder defaultBuilderState
    count' <- Data.Either.Right (countBuilderState st)
    page' <- Data.Either.Right (pageBuilderState st)
    all'' <- Data.Either.Right (all'BuilderState st)
    workspace_id' <- Data.Maybe.maybe (Data.Either.Left "Io.Superposition.Model.GetTypeTemplatesListInput.GetTypeTemplatesListInput.workspace_id is a required property.") Data.Either.Right (workspace_idBuilderState st)
    org_id' <- Data.Maybe.maybe (Data.Either.Left "Io.Superposition.Model.GetTypeTemplatesListInput.GetTypeTemplatesListInput.org_id is a required property.") Data.Either.Right (org_idBuilderState st)
    Data.Either.Right (GetTypeTemplatesListInput { 
        count = count',
        page = page',
        all' = all'',
        workspace_id = workspace_id',
        org_id = org_id'
    })


